use std::cmp::Ordering;

use super::*;

use async_trait::async_trait;
use fhir_model::{Order, OrderedSearchParameter, Resolve, SearchableResource};
use ordered_stream::{FromStream, OrderedStream};

/// An ordered FHIR search
#[derive(Debug, Clone)]
pub struct OrderedSearch<S, O> {
	/// The search to execute
	search: S,

	/// The parameter to order by
	order_by: OrderedSearchParameter<O>,
}

impl<S, O> Hash for OrderedSearch<S, O>
where
	S: Hash,
	O: Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.search.hash(state);
		self.order_by.hash(state);
	}
}

impl<S, O> PartialEq for OrderedSearch<S, O>
where
	S: PartialEq,
	O: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.search == other.search && self.order_by == other.order_by
	}
}

impl<S, O> Eq for OrderedSearch<S, O>
where
	S: Eq,
	O: Eq,
{
}

#[async_trait]
impl<E, R> ExecutableSearch<E, R> for OrderedSearch<Search<E, R>, R::Params>
where
	R: SearchableResource + Resolve + Clone + Send + Eq + 'static,
	R::Params: Clone + Eq + Send + Sync,
	E: SearchExecutor<R> + Send,
	E::Stream: Send,
{
	type Value = OrderedResourceStream<
		FromStream<
			E::Stream,
			Box<
				dyn FnMut(
						<E::Stream as Stream>::Item,
					) -> (OrderedSearchResult<R>, <E::Stream as Stream>::Item)
					+ Send
					+ Sync,
			>,
			OrderedSearchResult<R>,
		>,
	>;

	fn with_executor(mut self, executor: E) -> Self {
		self.search.executor = Some(executor);
		self
	}

	async fn send(self) -> Result<Self::Value, Error> {
		let stream = self.search.send().await?;

		let get_ordering = move |res: &Result<R, Error>| match res {
			Ok(r) => OrderedSearchResult::Orderable(r.clone(), self.order_by.clone()),
			Err(_) => OrderedSearchResult::Err,
		};

		let split_item: Box<
			dyn FnMut(
					<E::Stream as Stream>::Item,
				) -> (OrderedSearchResult<R>, <E::Stream as Stream>::Item)
				+ Send
				+ Sync,
		> = Box::new(move |data| {
			let ordering = get_ordering(&data);

			(ordering, data)
		});

		let ordered = FromStream::new(stream, split_item);

		Ok(OrderedResourceStream { stream: ordered })
	}
}

pin_project_lite::pin_project! {
	pub struct OrderedResourceStream<S> {
		#[pin]
		stream: S,
	}
}

impl<S> OrderedStream for OrderedResourceStream<S>
where
	S: OrderedStream,
{
	type Ordering = S::Ordering;

	type Data = S::Data;

	fn poll_next_before(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
		before: Option<&Self::Ordering>,
	) -> std::task::Poll<ordered_stream::PollResult<Self::Ordering, Self::Data>> {
		self.project().stream.poll_next_before(cx, before)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.stream.size_hint()
	}

	fn position_hint(&self) -> Option<ordered_stream::MaybeBorrowed<'_, Self::Ordering>> {
		self.stream.position_hint()
	}
}

impl<S> Stream for OrderedResourceStream<S>
where
	S: OrderedStream,
{
	type Item = S::Data;

	fn poll_next(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> std::task::Poll<Option<Self::Item>> {
		self.project().stream.poll_next_before(cx, None).map(|r| r.into_data())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.stream.size_hint()
	}
}

impl<E, R> Search<E, R>
where
	R: SearchableResource,
{
	/// Add an "_order" search parameter to the request. When executed, returns an [OrderedStream].
	pub fn order_by(
		mut self,
		parameter: OrderedSearchParameter<R::Params>,
	) -> OrderedSearch<Self, R::Params> {
		self.params.add_raw("_sort", parameter.to_string());

		OrderedSearch { search: self, order_by: parameter }
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderedSearchResult<R: SearchableResource> {
	/// A successful search result that has a value for the field(s) corresponding
	/// to the _sort search parameter.
	Orderable(R, OrderedSearchParameter<R::Params>),

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
				let Some(f1) = r1.resolve(&o.0) else {
					return flip(Ordering::Less, &o.1);
				};

				let Some(f2) = r2.resolve(&o.0) else {
					return flip(Ordering::Greater, &o.1);
				};

				f1.cmp(&f2)
			}
			(OrderedSearchResult::Orderable(_, _), _) => Ordering::Greater,
			(OrderedSearchResult::Err, OrderedSearchResult::Orderable(_, _)) => Ordering::Less,
			_ => Ordering::Equal,
		}
	}
}

fn flip(ordering: Ordering, order: &Order) -> Ordering {
	match order {
		Order::Ascending => ordering,
		Order::Descending => ordering.reverse(),
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
