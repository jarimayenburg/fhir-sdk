//! Search handling.

use std::marker::PhantomData;

use async_trait::async_trait;
use futures::{Future, Stream};
use reqwest::Url;

use super::{Client, Error};

/// A FHIR search that automatically resolves next pages
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnpagedSearch<E, R> {
	/// The executor of the search (e.g. the [Client])
	executor: Option<E>,

	/// Search parameters.
	params: SearchParameters,

	resource_type: PhantomData<R>,
}

impl<E, R> UnpagedSearch<E, R>
where
	E: UnpagedSearchExecutor<R>,
{
	/// Creates a new [UnpagedSearch] without an executor. An executor needs to be set with [UnpagedSearch::with_executor] before calling [UnpagedSearch::send].
	/// If [UnpagedSearch::send] is called without an executor being set, it will panic.
	///
	/// An [UnpagedSearch] is useful if you're looking for a way to represent a search that can be executed on an arbitrary or currently unknown [Client].
	/// If you just want to have your [Client] do a search, you are probably looking for [Client::search] instead.
	pub fn new() -> Self {
		Self { executor: None, params: SearchParameters::empty(), resource_type: PhantomData }
	}

	/// Sets a search executor (i.e. a [Client])
	pub fn with_executor(mut self, executor: E) -> Self {
		self.executor = Some(executor);
		self
	}

	/// Add a search parameter
	pub fn with<P>(mut self, field: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.params.add(field, parameter);
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
	pub fn and<P>(self, field: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.with(field, parameter)
	}

	/// Include other query parameters
	pub fn include<Q>(mut self, query: Q) -> Self
	where
		Q: IntoQuery,
	{
		self.params.add_query(query);
		self
	}

	/// Add a search parameter as a string. Alias of [Search::with_raw].
	///
	/// Prefer [Search::and] if possible.
	pub fn and_raw(self, key: impl Into<String>, value: impl ToString) -> Self {
		self.with_raw(key, value)
	}

	/// Execute the search
	pub async fn send(self) -> Result<E::Stream, Error> {
        dbg!(&self.params);
		self.executor.expect("no search executor set").search_unpaged(self.params).await
	}
}

impl<E, R> UnpagedSearch<E, R>
where
	E: UnpagedSearchExecutor<R> + PagedSearchExecutor<R>,
{
	/// Transform this search into a paged search
	pub fn paged(self, page_size: Option<u32>) -> PagedSearch<E, R> {
		let Self { executor, params, resource_type } = self;

		PagedSearch { executor, params, resource_type, page_size }
	}
}

#[derive(Debug)]
pub struct PagedSearch<E, R> {
	/// The executor of the search (e.g. the [Client])
	executor: Option<E>,

	/// Search parameters.
	params: SearchParameters,

	/// Preferred page size
	page_size: Option<u32>,

	resource_type: PhantomData<R>,
}

impl<E, R> PagedSearch<E, R>
where
	E: PagedSearchExecutor<R>,
{
	/// Execute the search
	pub async fn send(self) -> Result<(E::Stream, Option<NextPageCursor<E, R>>), Error> {
		self.executor
			.expect("no search executor set")
			.search_paged(self.params, self.page_size)
			.await
	}
}

/// Stream of resources returned by [Search::send].
pub trait Paged<R>: Stream<Item = Result<R, Error>> + Send + Sized {
	/// If the search is paged, returns the next page. Returns `None` if the [SearchResponse] is
	/// unpaged or if there is no next page available.
	fn next_page(&self) -> Option<impl Future<Output = Result<Self, Error>> + 'static>;
}

#[derive(Debug)]
pub struct NextPageCursor<E, R> {
	executor: E,
	next_page_url: Url,
	resource_type: PhantomData<R>,
}

impl<E, R> NextPageCursor<E, R>
where
	E: PagedSearchExecutor<R> + 'static,
{
	pub fn new(executor: E, next_page_url: Url) -> Self {
		Self { executor, next_page_url, resource_type: PhantomData }
	}

	pub async fn next_page(self) -> Result<(E::Stream, Option<Self>), Error> {
		self.executor.fetch_next_page(self.next_page_url).await
	}
}

/// Executor of unpaged FHIR searches (e.g. [Client])
#[async_trait]
pub trait UnpagedSearchExecutor<R>: Sized {
	/// The stream of FHIR resources that will be returned
	type Stream: Stream<Item = Result<R, Error>>;

	/// Execute an unpaged search
	async fn search_unpaged(self, params: SearchParameters) -> Result<Self::Stream, Error>;
}

/// Executor of paged FHIR searches (e.g. [Client])
#[async_trait]
pub trait PagedSearchExecutor<R>: Sized {
	/// The stream of FHIR resources that will be returned
	type Stream: Stream<Item = Result<R, Error>>;

	async fn search_paged(
		self,
		params: SearchParameters,
		page_size: Option<u32>,
	) -> Result<(Self::Stream, Option<NextPageCursor<Self, R>>), Error>;

	async fn fetch_next_page(
		self,
		next_page_url: Url,
	) -> Result<(Self::Stream, Option<NextPageCursor<Self, R>>), Error>;
}

impl<V: 'static> Client<V> {
	/// Start constructing a search for FHIR resources of a given type.
	/// Only returns matches. Populates reference target fields with any
	/// matching included resources.
	pub fn search<R>(&self) -> UnpagedSearch<Self, R>
	where
		Self: UnpagedSearchExecutor<R>,
		R: Send,
	{
		UnpagedSearch::new().with_executor(self.clone())
	}
}

/// A collection of AND-joined search parameters.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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
	pub fn with<P>(field: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		Self::empty().and(field, parameter)
	}

	/// Add a search parameter.
	#[must_use]
	pub fn and<P>(mut self, key: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.add(key, parameter);
		self
	}

	/// Add other types of parameters to the query string
	pub fn include<Q>(mut self, query: Q) -> Self
	where
		Q: IntoQuery,
	{
		self.add_query(query);
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

	fn add<P>(&mut self, key: &str, parameter: P)
	where
		P: SearchParameter,
	{
		self.add_query((key, parameter))
	}

	fn add_query<Q>(&mut self, query: Q)
	where
		Q: IntoQuery,
	{
		let (key, value) = query.into_query();

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

impl<Q> FromIterator<Q> for SearchParameters
where
	Q: IntoQuery,
{
	fn from_iter<T: IntoIterator<Item = Q>>(iter: T) -> Self {
		iter.into_iter().fold(SearchParameters::empty(), |params, param| params.include(param))
	}
}

/// Functionality to convert a search parameter to the URL query.
pub trait IntoQuery {
	/// Convert this search parameter into the query pair (key, value).
	fn into_query(self) -> (String, String);
}

impl<S: SearchParameter> IntoQuery for (&str, S) {
	fn into_query(self) -> (String, String) {
		let (key, param) = self;

		let key = if let Some(modifier) = param.modifier() {
			format!("{key}:{modifier}")
		} else {
			key.to_string()
		};

		(key, param.query_value())
	}
}

/// A FHIR search parameter
pub trait SearchParameter: Sized {
	/// URL safe query string value of the parameter
	fn query_value(&self) -> String;

	/// Modifier of the parameter, like :in, :not-in. Without the :
	fn modifier(&self) -> Option<&str> {
		None
	}

	/// Or this parameter with another parameter
	fn or(self, param: Self) -> SearchParameterOrList<Self> {
		SearchParameterOrList::new(self).or(param)
	}
}

/// A set of OR-ed FHIR search parameters
#[derive(Debug)]
pub struct SearchParameterOrList<P> {
	// Used to validate that every added parameter has the same modifier
	modifier: Option<String>,

	// The parameters that should be OR'ed
	params: Vec<P>,
}

impl<P: SearchParameter> SearchParameterOrList<P> {
	fn empty() -> Self {
		Self { modifier: None, params: Vec::new() }
	}

	/// Create a new [SearchParameterOrList]
	pub fn new(param: P) -> Self {
		Self { modifier: param.modifier().map(|m| m.to_string()), params: vec![param] }
	}

	/// Add an OR search parameter
	pub fn or(mut self, param: P) -> Self {
		if self.modifier.as_deref() != param.modifier() {
			panic!("cannot OR two search parameters with different modifiers");
		}

		self.params.push(param);
		self
	}
}

impl<P: SearchParameter> SearchParameter for SearchParameterOrList<P> {
	fn modifier(&self) -> Option<&str> {
		self.modifier.as_deref()
	}

	fn query_value(&self) -> String {
		self.params.iter().map(|p| p.query_value()).collect::<Vec<String>>().join(",")
	}
}

impl<P> FromIterator<P> for SearchParameterOrList<P>
where
	P: SearchParameter,
{
	fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
		iter.into_iter().fold(SearchParameterOrList::empty(), |ors, param| ors.or(param))
	}
}

/// Escape a search parameter value.
pub(crate) fn escape_value(value: &str) -> String {
	value.replace('\\', "\\\\").replace('|', "\\|").replace('$', "\\$").replace(',', "\\,")
}

#[cfg(test)]
mod tests {
	use super::*;

	struct MyParam<'a> {
		modifier: Option<&'a str>,
		value: &'a str,
	}

	impl SearchParameter for MyParam<'_> {
		fn modifier(&self) -> Option<&str> {
			self.modifier
		}

		fn query_value(&self) -> String {
			self.value.to_string()
		}
	}

	#[test]
	fn or() {
		let params =
			MyParam { modifier: None, value: "bla" }.or(MyParam { modifier: None, value: "ble" });

		assert_eq!(params.query_value(), "bla,ble");
	}

	#[test]
	#[should_panic]
	fn or_different_modifiers() {
		MyParam { modifier: None, value: "bla" }
			.or(MyParam { modifier: Some("mod"), value: "ble" });
	}
}
