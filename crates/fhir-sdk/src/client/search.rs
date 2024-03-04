//! Search handling.

use std::marker::PhantomData;

use futures::Stream;

use super::{Client, Error};

#[derive(Debug, Clone)]
pub struct Search<E, R> {
	/// The executor of the search (e.g. the [Client])
	executor: E,

	/// Search parameters.
	params: SearchParameters,

	/// Paging information
	paging: Paging,

	resource_type: PhantomData<R>,
}

impl<E, R> Search<E, R>
where
	E: SearchExecutor<R>,
{
	pub fn new(executor: E) -> Self {
		Self {
			executor,
			params: SearchParameters::empty(),
			paging: Paging::default(),
			resource_type: PhantomData,
		}
	}

	/// Add a search parameter
	pub fn with<P>(mut self, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.params.add(parameter);
		self
	}

	/// Add an existing set of search parameters
	pub fn with_params(mut self, parameters: SearchParameters) -> Self {
		self.params.add_all(parameters);
		self
	}

	/// Add a search parameter as a raw string.
	///
	/// Prefer [Search::with] if possible.
	pub fn with_raw(mut self, key: impl Into<String>, value: impl ToString) -> Self {
		self.params.add_raw(key, value);
		self
	}

	/// Add a search parameter. Alias of [Search::with].
	pub fn and<P>(self, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.with(parameter)
	}

	/// Add a search parameter as a string. Alias of [Search::with_raw].
	///
	/// Prefer [Search::and] if possible.
	pub fn and_raw(self, key: impl Into<String>, value: impl ToString) -> Self {
		self.with_raw(key, value)
	}

	/// Make the resulting [SearchResponse] paged. Next pages can be fetched using
	/// [SearchResponse::next_page].
	pub fn paged(mut self, page_size: u32) -> Self {
		self.paging = Paging::Paged { page_size };
		self
	}

	/// Execute the search
	pub fn send(self) -> impl SearchResponse<R> {
		self.executor.execute_search(self.params, self.paging)
	}
}

/// Describes how to handle paging in the search response
#[derive(Clone, Debug, Default)]
pub enum Paging {
	/// Pages from the resulting [SearchResponse] should be resolved as the stream is consumed
	/// automatically fetching next pages and putting the matches on the stream
	#[default]
	Unpaged,

	/// The resulting [SearchResponse] should be paged. The stream will be limited to the size
	/// of the page and [SearchResponse::next_page] should be called to fetch the next page.
	Paged { page_size: u32 },
}

/// Stream of resources returned by [Search::send].
pub trait SearchResponse<R>: Stream<Item = Result<R, Error>> + Send + Sized {
	/// If the search is paged, returns the next page. Returns `None` if the [SearchResponse] is
	/// unpaged or if there is no next page available.
	fn next_page(&self) -> Option<Self>;
}

pub trait SearchExecutor<R>: Sized {
	fn execute_search(self, params: SearchParameters, paging: Paging) -> impl SearchResponse<R>;
}

impl<V> Client<V> {
	/// Start constructing a search for FHIR resources of a given type.
	/// Only returns matches. Populates reference target fields with any
	/// matching included resources.
	pub fn search<R>(&self) -> Search<Self, R>
	where
		Self: SearchExecutor<R>,
	{
		Search::new(self.clone())
	}
}

/// A collection of AND-joined search parameters.
#[derive(Debug, Default, Clone)]
pub struct SearchParameters {
	/// List of search queries.
	queries: Vec<(String, String)>,
}

impl SearchParameters {
	/// Create a new list of [`SearchParameters`].
	#[must_use]
	pub fn empty() -> Self {
		Self::default()
	}

	/// Initialize a new [`SearchParameters`] with a parameter
	pub fn with<P>(parameter: P) -> Self
	where
		P: SearchParameter,
	{
		Self::empty().and(parameter)
	}

	/// Add a search parameter.
	#[must_use]
	pub fn and<P>(mut self, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.add(parameter);
		self
	}

	/// Add a raw custom search parameter.
	///
	/// The key of the search query includes modifiers or chaining.
	///
	/// The value of the search query might include multiple comma-separated
	/// values. A value can consist of pipe-separated values for token search or
	/// can be prepended by a comparator like `lt` for numbers.
	#[must_use]
	pub fn and_raw(mut self, key: impl Into<String>, value: impl ToString) -> Self {
		self.add_raw(key, value);
		self
	}

	/// Convert to a list of raw queries.
	pub(crate) fn into_queries(self) -> Vec<(String, String)> {
		self.queries
	}

	fn add<P>(&mut self, parameter: P)
	where
		P: SearchParameter,
	{
		let (key, value) = parameter.into_query();

		self.add_raw(key, value);
	}

	fn add_all(&mut self, parameters: SearchParameters) {
		for (key, value) in parameters.into_queries() {
			self.add_raw(key, value);
		}
	}

	fn add_raw(&mut self, key: impl Into<String>, value: impl ToString) {
		self.queries.push((key.into(), value.to_string()));
	}
}

/// Functionality to convert a SearchParameter to the URL query.
pub trait SearchParameter {
	/// Convert this search parameter into the query pair (key, value).
	fn into_query(self) -> (String, String);
}

/// Escape a search parameter value.
pub(crate) fn escape_value(value: &str) -> String {
	value.replace('\\', "\\\\").replace('|', "\\|").replace('$', "\\$").replace(',', "\\,")
}
