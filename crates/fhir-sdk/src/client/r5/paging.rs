//! FHIR paging functionality, e.g. for search results.

use std::{collections::VecDeque, pin::Pin, task::Poll};

use fhir_model::r5::{
	codes::{BundleType, LinkRelationTypes, SearchEntryMode},
	resources::{Bundle, BundleEntry, DomainResource, NamedResource, Resource, TypedResource},
};
use futures::{future::BoxFuture, ready, FutureExt, Stream, StreamExt};
use reqwest::Url;

use crate::client::r5::references::populate_reference_targets;

use super::{Client, DeserializableResource, Error, FhirR5};

/// Results of a query that can be paged or given via URL only. The resources
/// can be consumed via the `Stream`/`StreamExt` traits.
pub struct Paged<R> {
	/// The FHIR client to make further requests for the next pages.
	client: Client<FhirR5>,
	/// The URL of the next page. This is opaque and can be anything the server
	/// wants. The client ensures it accesses the same server only.
	next_url: Option<Url>,
	/// The current set of search matches.
	matches: Option<SearchMatches<R>>,
	/// Current future to retrieve the next page.
	future_next_page: Option<BoxFuture<'static, Result<Bundle, Error>>>,
}

impl<R: NamedResource> Paged<R> {
	/// Start up a new Paged<R> stream.
	pub(crate) fn new(client: Client<FhirR5>, url: Url) -> Self {
		let future_next_page = Some(fetch_resource(client.clone(), url).boxed());

		Self { client, next_url: None, matches: None, future_next_page }
	}
}

impl<R> Stream for Paged<R>
where
	R: NamedResource + DomainResource + DeserializableResource + 'static,
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
			tracing::debug!("Paged::matches is set, polling for next match");
			if let Poll::Ready(res) = matches.poll_next_unpin(cx) {
				if let Some(r) = res {
					tracing::debug!("Next match in Paged::matches available");
					return Poll::Ready(Some(r));
				} else {
					tracing::debug!("Paged::matches is empty, waiting for next page");
					self.matches = None;
				}
			}
		// If there are no more matches and there is a next page future, check if it's ready
		} else if let Some(future_next_page) = self.future_next_page.as_mut() {
			tracing::debug!("Paged::future_next_page is set, polling for next page");
			if let Poll::Ready(next_page) = future_next_page.as_mut().poll(cx) {
				self.future_next_page = None;
				tracing::debug!("Next page fetched and ready");

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
				self.matches = Some(SearchMatches::from_searchset(self.client.clone(), bundle));
			}
		// Start retrieving the next page if we have a next URL and there is no next page being fetched.
		} else if let Some(next_url) = self.next_url.as_ref() {
			tracing::debug!("Current page has next URL, starting to fetch next page");
			self.future_next_page =
				Some(fetch_resource(self.client.clone(), next_url.clone()).boxed());
			self.next_url = None;
		}

		// Else check if all resources were consumed or if we are waiting for new
		// resources to arrive.
		if self.matches.is_some() {
			tracing::debug!("Paged results waiting for remaining resources in current page");
			cx.waker().wake_by_ref();
			Poll::Pending
		} else if self.future_next_page.is_some() {
			tracing::debug!("Paged results waiting for response to next URL fetch");
			cx.waker().wake_by_ref();
			Poll::Pending
		} else if self.next_url.is_some() {
			tracing::debug!("Paged results waiting to fetch next URL");
			cx.waker().wake_by_ref();
			Poll::Pending
		} else {
			tracing::debug!("Paged results exhausted");
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
	bundle
		.link
		.iter()
		.flatten()
		.find(|link| link.relation == LinkRelationTypes::Next)
		.map(|link| &link.url)
}

/// Query a resource from a given URL.
async fn fetch_resource<R: DeserializableResource>(
	client: Client<FhirR5>,
	url: Url,
) -> Result<R, Error> {
	let url_str = url.to_string();

	// Make sure we are not forwarded to any malicious server.
	if url.origin() != client.0.base_url.origin() {
		return Err(Error::DifferentOrigin(url_str));
	}

	// Fetch a single resource from the given URL.
	let resource = client.read_generic(url).await?;
	resource.ok_or_else(|| Error::ResourceNotFound(url_str))
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
	client: Client<FhirR5>,
	bundle: Bundle,
	matches: VecDeque<BundleEntry>,
	future_resource: Option<BoxFuture<'static, Result<R, Error>>>,
}

impl<R> SearchMatches<R>
where
	R: NamedResource + DomainResource + TryFrom<Resource>,
{
	pub fn from_searchset(client: Client<FhirR5>, bundle: Bundle) -> SearchMatches<R> {
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

		Self { client, bundle, matches, future_resource: None }
	}
}

impl<R> Stream for SearchMatches<R>
where
	R: NamedResource + DomainResource + DeserializableResource + 'static,
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
					populate_reference_targets(
						&self.client.0.base_url,
						&mut resource,
						Some(&self.bundle),
						None,
					);

					Poll::Ready(Some(Ok(resource)))
				}
				Err(e) => Poll::Ready(Some(Err(e))),
			};
		}

		// Otherwise get the next match from the list
		while let Some(entry) = self.matches.pop_front() {
			if let Some(resource) = entry.resource {
				tracing::debug!("Found next Bundle entry to return");
				let resource_type = resource.resource_type();

				let mut r: R = resource.try_into().map_err(|_| {
					Error::WrongResourceType(resource_type.to_string(), R::TYPE.to_string())
				})?;

				populate_reference_targets(
					&self.client.0.base_url,
					&mut r,
					Some(&self.bundle),
					None,
				);

				return Poll::Ready(Some(Ok(r)));
			} else if let Some(url) = entry.full_url {
				if let Ok(url) = Url::parse(&url) {
					tracing::debug!("Next entry needs to be fetched, starting to fetch it");

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
