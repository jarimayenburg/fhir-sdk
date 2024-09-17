use std::cmp::Ordering;

use super::*;

use fhir_model::{Resolve, ResourceSearchParameterDefinition, SearchableResource};
use ordered_stream::{FromStream, OrderedStream};

#[derive(Debug)]
pub struct OrderedSearch<S, O> {
	/// The search to execute
	search: S,

	/// The parameter to order by
	order_by: O,
}

impl<E, R> OrderedSearch<UnpagedSearch<E, R>, R::Params>
where
	R: SearchableResource + Resolve + Clone + Eq + 'static,
	R::Params: Clone + Eq,
	E: UnpagedSearchExecutor<R>,
{
	pub async fn send(
		self,
	) -> Result<impl OrderedStream<Data = <E::Stream as Stream>::Item>, Error> {
		let stream = self.search.send().await?;

		Ok(FromStream::with_ordering(stream, move |res| match res {
			Ok(r) => OrderedSearchResult::Orderable(r.clone(), self.order_by.clone()),
			Err(_) => OrderedSearchResult::Err,
		}))
	}
}

impl<E, R> UnpagedSearch<E, R>
where
	R: SearchableResource,
{
	/// Add an "_order" search parameter to the request. When executed, returns an [OrderedStream].
	pub fn order_by(mut self, parameter: R::Params) -> OrderedSearch<Self, R::Params> {
		self.params.add_raw("_sort", parameter.code());

		OrderedSearch { search: self, order_by: parameter }
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderedSearchResult<R: SearchableResource> {
	/// A successful search result that has a value for the field(s) corresponding
	/// to the _sort search parameter.
	Orderable(R, R::Params),

	/// A search error
	Err,
}

/// Implement a total ordering, ranking resources that have no value for
/// the ordering field lower than resources that do. Search errors are
/// ranked the lowest.
impl<R> Ord for OrderedSearchResult<R>
where
	R: SearchableResource + Resolve + Eq,
	R::Params: Eq,
{
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(OrderedSearchResult::Orderable(r1, o), OrderedSearchResult::Orderable(r2, _)) => {
				let Some(f1) = r1.resolve(&o) else {
					return Ordering::Less;
				};

				let Some(f2) = r2.resolve(&o) else {
					return Ordering::Greater;
				};

				f1.cmp(&f2)
			}
			(OrderedSearchResult::Orderable(_, _), _) => Ordering::Greater,
			(OrderedSearchResult::Err, OrderedSearchResult::Orderable(_, _)) => Ordering::Less,
			_ => Ordering::Equal,
		}
	}
}

impl<R> PartialOrd for OrderedSearchResult<R>
where
	R: SearchableResource + Resolve + Eq,
	R::Params: PartialEq + Eq,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
