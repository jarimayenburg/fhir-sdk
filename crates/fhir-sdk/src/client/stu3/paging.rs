//! FHIR paging functionality, e.g. for search results.

use std::{collections::VecDeque, pin::Pin, task::Poll};

use fhir_model::{
	stu3::{
		codes::{BundleType, SearchEntryMode},
		resources::{Bundle, BundleEntry, NamedResource, Resource, ResourceType},
	},
	ParsedReference,
};
use futures::{future::BoxFuture, ready, FutureExt, Stream, StreamExt};
use reqwest::Url;
use serde::de::DeserializeOwned;

use super::{Client, Error, FhirStu3};

/// Results of a query that can be paged or given via URL only. The resources
/// can be consumed via the `Stream`/`StreamExt` traits.
pub struct Paged<R> {
	/// The FHIR client to make further requests for the next pages.
	client: Client<FhirStu3>,
	/// The URL of the next page. This is opaque and can be anything the server
	/// wants. The client ensures it accesses the same server only.
	next_url: Option<Url>,
	/// The current set of search matches.
	matches: Option<SearchMatches<R>>,
	/// The resource type to match on. None for system searches.
	resource_type: Option<ResourceType>,
	/// Current future to retrieve the next page.
	future_next_page: Option<BoxFuture<'static, Result<Bundle, Error>>>,
}

impl<R: NamedResource> Paged<R> {
	/// Start up a new Paged<R> stream.
	pub(crate) fn new_typed(client: Client<FhirStu3>, url: Url) -> Self {
		Self::new(client, url, Some(R::TYPE))
	}
}

impl Paged<Resource> {
	pub(crate) fn new_system(client: Client<FhirStu3>, url: Url) -> Self {
		Self::new(client, url, None)
	}
}

impl<R> Paged<R> {
	fn new(client: Client<FhirStu3>, url: Url, resource_type: Option<ResourceType>) -> Self {
		let future_next_page = Some(fetch_resource(client.clone(), url).boxed());

		Self { client, next_url: None, matches: None, resource_type, future_next_page }
	}
}

impl<R> Stream for Paged<R>
where
	R: TryFrom<Resource> + DeserializeOwned + 'static,
	for<'a> &'a mut Resource: From<&'a mut R>,
{
	type Item = Result<R, Error>;

	fn poll_next(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let span = tracing::trace_span!("Paged::poll_next");
		let _span_guard = span.enter();

		// If there are still matches left, get the next one
		if let Some(matches) = self.matches.as_mut() {
			if let Poll::Ready(res) = matches.poll_next_unpin(cx) {
				if let Some(r) = res {
					return Poll::Ready(Some(r));
				} else {
					self.matches = None;
				}
			}
		// If there are no more matches and there is a next page future, check if it's ready
		} else if let Some(future_next_page) = self.future_next_page.as_mut() {
			if let Poll::Ready(next_page) = future_next_page.as_mut().poll(cx) {
				self.future_next_page = None;
				tracing::trace!("Next page fetched and ready");

				// Get the Bundle or error out.
				let bundle = match next_page {
					Ok(bundle) => bundle,
					Err(err) => return Poll::Ready(Some(Err(err))),
				};

				// Parse the next page's URL or error out.
				if let Some(next_url_string) = find_next_page_url(&bundle) {
					let Ok(next_url) = Url::parse(next_url_string) else {
						tracing::error!("Could not parse next page URL");
						return Poll::Ready(Some(Err(Error::UrlParse(next_url_string.clone()))));
					};
					self.next_url = Some(next_url);
				}

				// Save the `BundleEntry`s.
				self.matches = Some(SearchMatches::from_searchset(
					self.client.clone(),
					self.resource_type,
					bundle,
				));
			}
		// Start retrieving the next page if we have a next URL and there is no next page being fetched.
		} else if let Some(next_url) = &self.next_url {
			tracing::trace!("Current page has next URL, starting to fetch next page");
			self.future_next_page =
				Some(fetch_resource(self.client.clone(), next_url.clone()).boxed());
			cx.waker().wake_by_ref();
		}

		// Else check if all resources were consumed or if we are waiting for new
		// resources to arrive.
		if self.matches.is_some() {
			tracing::trace!("Paged results waiting for remaining resources in current page");
			Poll::Pending
		} else if self.future_next_page.is_some() {
			tracing::trace!("Paged results waiting for response to next URL fetch");
			Poll::Pending
		} else if self.next_url.is_some() {
			tracing::trace!("Paged results waiting to fetch next URL");
			cx.waker().wake_by_ref();
			Poll::Pending
		} else {
			tracing::trace!("Paged results exhausted");
			Poll::Ready(None)
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if let Some(matches) = &self.matches {
			let (page_min, page_max) = matches.size_hint();

			if self.next_url.is_some() {
				return (page_min, None);
			} else {
				return (page_min, page_max);
			}
		}

		(0, None)
	}
}

/// Find the URL of the next page of the results returned in the Bundle.
fn find_next_page_url(bundle: &Bundle) -> Option<&String> {
	bundle.link.iter().flatten().find(|link| link.relation == "next").map(|link| &link.url)
}

/// Query a resource from a given URL.
async fn fetch_resource<R: DeserializeOwned>(
	client: Client<FhirStu3>,
	url: Url,
) -> Result<R, Error> {
	// Make sure we are not forwarded to any malicious server.
	if url.origin() != client.0.base_url.origin() {
		return Err(Error::DifferentOrigin(url.to_string()));
	}

	// Fetch a single resource from the given URL.
	let resource = client.read_generic(url.clone()).await?;
	resource.ok_or_else(|| Error::ResourceNotFound(url.to_string()))
}

impl<R> std::fmt::Debug for Paged<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Paged")
			.field("client", &self.client)
			.field("next_url", &self.next_url)
			.field("matches", &self.matches.as_ref().map(|_| "_"))
			.field("future_next_page", &self.future_next_page.as_ref().map(|_| "_"))
			.finish()
	}
}

/// Stream that yields each match entry in a searchset Bundle. Tries to fetch entries from the fullUrl property
/// if the resource field is empty. Fills resource reference target fields with the referred to resources,
/// if available in the Bundle.
struct SearchMatches<R> {
	client: Client<FhirStu3>,
	bundle: Bundle,
	matches: VecDeque<BundleEntry>,
	resource_type: Option<ResourceType>,
	future_resource: Option<BoxFuture<'static, Result<R, Error>>>,
}

impl<R> SearchMatches<R>
where
	R: TryFrom<Resource>,
	for<'a> &'a mut Resource: From<&'a mut R>,
{
	pub fn from_searchset(
		client: Client<FhirStu3>,
		resource_type: Option<ResourceType>,
		bundle: Bundle,
	) -> SearchMatches<R> {
		assert!(
			bundle.r#type == BundleType::Searchset,
			"unable to get search matches from non-searchset Bundles"
		);

		// I hate cloning here but I didn't want to have to add lifetime params all over the place.
		// We shall see if it affects performance.
		let matches: VecDeque<_> = bundle
			.entry
			.iter()
			.flatten()
			.filter(|e| {
				e.search
					.as_ref()
					.and_then(|s| s.mode.as_ref())
					.map_or(true, |m| *m == SearchEntryMode::Match)
			})
			.cloned()
			.collect();

		Self { client, bundle, matches, resource_type, future_resource: None }
	}

	fn populate_reference_targets(&self, r: &mut R) {
		let resource: &mut Resource = r.into();

		let resource_type_and_id = format!(
			"{}/{}",
			resource.resource_type(),
			resource.as_base_resource().id().clone().unwrap_or("<unknown>".to_string())
		);

		let contained = resource.as_domain_resource().map(|dr| dr.contained().clone());

		for field in resource.as_base_resource_mut().lookup_references() {
			if let Some(reference) = field.reference().clone().parse() {
				match reference {
					// (Only if the resource is a domain resource) Look up local references in the resource's own contained field.
					ParsedReference::Local { id } => {
						if let Some(ref contained) = contained {
							if let Some(target) = contained
								.iter()
								.find(|c| c.as_base_resource().id() == &Some(id.to_string()))
							{
								if field.set_target(target.clone()).is_err() {
									tracing::warn!("Reference field in {} refers to contained resource {} of unsupported type {}", resource_type_and_id, id, target.resource_type().to_string());
								}
							} else {
								tracing::warn!(
                                    "Resource {} in Bundle refers to missing contained resource #{}",
                                    resource_type_and_id,
                                    id
                                );
							}
						}
					}
					other => {
						if let Some(target) =
							self.bundle.resolve_reference(self.client.0.base_url.as_str(), &other)
						{
							if field.set_target(target.clone()).is_err() {
								tracing::warn!("Reference field in {} refers to resource {} of unsupported type {}", resource_type_and_id, other.to_string(), other.resource_type().unwrap().to_string());
							}
						} else {
							tracing::debug!(
                                "Resource {} in Bundle refers to resource {}, which is not in the Bundle",
                                resource_type_and_id,
                                other.to_string()
                            );
						}
					}
				}
			}
		}
	}
}

impl<R> Stream for SearchMatches<R>
where
	R: TryFrom<Resource> + DeserializeOwned + 'static,
	for<'a> &'a mut Resource: From<&'a mut R>,
{
	type Item = Result<R, Error>;

	fn poll_next(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let span = tracing::trace_span!("SearchMatches::poll_next");
		let _span_guard = span.enter();

		// Check on resource fetch future first to output as the next resource.
		if let Some(future_resource) = self.future_resource.as_mut() {
			return match ready!(future_resource.as_mut().poll(cx)) {
				Ok(mut resource) => {
					tracing::trace!("Next `fullUrl` fetched resource ready");

					self.future_resource = None;
					self.populate_reference_targets(&mut resource);

					Poll::Ready(Some(Ok(resource)))
				}
				Err(e) => Poll::Ready(Some(Err(e))),
			};
		}

		// Otherwise get the next match from the list
		while let Some(entry) = self.matches.pop_front() {
			if let Some(resource) = entry.resource {
				let resource_type = resource.resource_type();

				let mut r: R = resource.try_into().map_err(|_| {
					Error::WrongResourceType(
						resource_type.to_string(),
						// Unwrapping here should be fine since resource_type will always be set when R: NamedResource
						// And if R is Resource, TryInto has error type Infallible and won't be called
						self.resource_type.unwrap().to_string(),
					)
				})?;

				self.populate_reference_targets(&mut r);

				return Poll::Ready(Some(Ok(r)));
			} else if let Some(url) = entry.full_url {
				if let Ok(url) = Url::parse(&url) {
					tracing::trace!("Next entry needs to be fetched, starting to fetch it");

					self.future_resource = Some(fetch_resource(self.client.clone(), url).boxed());
					cx.waker().wake_by_ref();

					return Poll::Pending;
				} else {
					tracing::error!("Could not parse next entry URL");

					return Poll::Ready(Some(Err(Error::UrlParse(url))));
				}
			}
		}

		tracing::trace!("SearchMatches exhausted");
		Poll::Ready(None)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.matches.len();

		(size, Some(size))
	}
}
