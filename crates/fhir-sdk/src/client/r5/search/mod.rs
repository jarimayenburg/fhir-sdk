//! Client search implementation.

use crate::client::{
	search::{Paging, SearchExecutor, SearchResponse},
	Client, Error, FhirR5, SearchParameters,
};
use fhir_model::r5::resources::{DomainResource, NamedResource, Resource};
use paging::Unpaged;

#[allow(unused_imports)]
pub use params::*;

mod paging;
mod params;

impl<R> SearchExecutor<R> for Client<FhirR5>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	fn execute_search(self, params: SearchParameters, paging: Paging) -> impl SearchResponse<R> {
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(params.into_queries()).finish();

		match paging {
			Paging::Unpaged => Unpaged::new(self, url),
			_ => panic!("Unsupported!"),
		}
	}
}
