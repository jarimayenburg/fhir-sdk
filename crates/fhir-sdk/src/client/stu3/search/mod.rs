//! Client search implementation.

use crate::client::search::NextPageCursor;
use crate::client::{
	search::PagedSearchExecutor, search::UnpagedSearchExecutor, Client, Error, FhirStu3,
	SearchParameters,
};
use async_trait::async_trait;
use fhir_model::stu3::resources::{Bundle, DomainResource, NamedResource, Resource};
use paging::{Page, Unpaged};

#[allow(unused_imports)]
pub use params::*;
use reqwest::Url;
use tracing::warn;

mod paging;
mod params;

#[async_trait]
impl<R> PagedSearchExecutor<R> for Client<FhirStu3>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	type Stream = Page<R>;

	#[allow(refining_impl_trait)]
	async fn search_paged(
		self,
		params: SearchParameters,
		page_size: Option<u32>,
	) -> Result<(Page<R>, Option<NextPageCursor<Self, R>>), Error> {
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(params.into_queries()).finish();

		if let Some(page_size) = page_size {
			url.query_pairs_mut().append_pair("_count", &page_size.to_string());
		}

		self.fetch_next_page(url).await
	}

	#[allow(refining_impl_trait)]
	async fn fetch_next_page(
		self,
		url: Url,
	) -> Result<(Page<R>, Option<NextPageCursor<Self, R>>), Error> {
		let searchset: Bundle = dbg!(self.fetch_resource(url).await)?;

		let cursor = match find_next_page_url(&searchset) {
			Some(Ok(u)) => Some(NextPageCursor::new(self.clone(), u)),
			Some(Err(e)) => {
				warn!("Unable to parse next page URL: {e}");

				None
			}
			_ => None,
		};

		let page = Page::from_searchset(self, searchset);

		Ok((page, cursor))
	}
}

#[async_trait]
impl<R> UnpagedSearchExecutor<R> for Client<FhirStu3>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	type Stream = Unpaged<R>;

	#[allow(refining_impl_trait)]
	async fn search_unpaged(self, params: SearchParameters) -> Result<Unpaged<R>, Error> {
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(params.into_queries()).finish();

		let searchset: Bundle = self.fetch_resource(dbg!(url.clone())).await?;

		Ok(Unpaged::from_searchset(self, searchset))
	}
}

/// Find the URL of the next page of the results returned in the Bundle.
pub(self) fn find_next_page_url(bundle: &Bundle) -> Option<Result<Url, Error>> {
	let url_str =
		bundle.link.iter().flatten().find(|link| link.relation == "next").map(|link| &link.url)?;

	Some(Url::parse(url_str).map_err(|_| Error::UrlParse(url_str.to_string())))
}

#[cfg(test)]
mod tests {
	use fhir_model::stu3::resources::Observation;

	use crate::client::{Client, FhirStu3};

	/// The search code is prone to run into rustc bugs [rust-lang/rust#100013](https://github.com/rust-lang/rust/issues/100013) and
	/// [rust-lang/rust#102211](https://github.com/rust-lang/rust/issues/102211). We implemented a workaround for it.
	/// This test is just there to prevent regressions. It doesn't actually test anything, we just need to make sure this compiles
	#[allow(dead_code)]
	async fn rustc_bug_workaround_inner() {
		let client: Client<FhirStu3> = Client::builder().build().unwrap();

		fn assert_send<T: Send>(v: T) -> T {
			v
		}

		// We don't actually test anything here, we just need to make sure this compiles
		let _ = assert_send(client.search::<Observation>().send());
	}
}
