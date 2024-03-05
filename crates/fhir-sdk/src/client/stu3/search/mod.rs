//! Client search implementation.

use crate::client::search::NextPageCursor;
use crate::client::{search::SearchExecutor, Client, Error, FhirStu3, SearchParameters};
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
impl<R> SearchExecutor<R> for Client<FhirStu3>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
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
		let searchset: Bundle = self.fetch_resource(url).await?;

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

	#[allow(refining_impl_trait)]
	async fn search_unpaged(self, params: SearchParameters) -> Result<Unpaged<R>, Error> {
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(params.into_queries()).finish();

		let searchset: Bundle = self.fetch_resource(url.clone()).await?;

		Ok(Unpaged::from_searchset(self, searchset))
	}
}

/// Find the URL of the next page of the results returned in the Bundle.
pub(self) fn find_next_page_url(bundle: &Bundle) -> Option<Result<Url, Error>> {
	let url_str =
		bundle.link.iter().flatten().find(|link| link.relation == "next").map(|link| &link.url)?;

	Some(Url::parse(url_str).map_err(|_| Error::UrlParse(url_str.to_string())))
}
