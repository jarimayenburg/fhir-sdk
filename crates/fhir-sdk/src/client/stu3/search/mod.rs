//! Client search implementation.

use crate::client::{search::SearchExecutor, Client, Error, FhirStu3, SearchParameters};
use fhir_model::stu3::resources::{DomainResource, NamedResource, Resource};
use futures::Stream;
use paging::Paged;

#[allow(unused_imports)]
pub use params::*;

mod paging;
mod params;

impl<R> SearchExecutor<R> for Client<FhirStu3>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	fn execute_search(
		self,
		params: SearchParameters,
	) -> impl Stream<Item = Result<R, Error>> + Send + 'static {
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(params.into_queries()).finish();

		Paged::new(self, url)
	}
}
