/// The ordering for a sorted search
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Order {
	/// Ascending order, does not add a "-" prefix to the parameter
	Ascending,

	/// Descending order, adds a "-" prefix to the parameter
	Descending,
}

/// Search parameter with ordering information
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrderedSearchParameter<P>(pub P, pub Order);

impl<P> ToString for OrderedSearchParameter<P>
where
	P: ResourceSearchParameterDefinition,
{
	fn to_string(&self) -> String {
		let code = self.0.code().to_string();

		match self.1 {
			Order::Ascending => code,
			Order::Descending => format!("-{code}"),
		}
	}
}

/// A resource type that can be searched on
pub trait SearchableResource {
	/// Enum of parameters that can be used in the search
	type Params: ResourceSearchParameterDefinition;
}

/// Trait for a resource's search parameter definition
pub trait ResourceSearchParameterDefinition {
	/// The resource type this search parameter can be used against
	fn resource_type(&self) -> &'static str;

	/// The code used in the URL or the parameter name in a parameters resource for this search parameter.
	fn code(&self) -> &'static str;

	/// Create an [OrderedSearchParameter] from this parameter
	fn order(self, order: Order) -> OrderedSearchParameter<Self>
	where
		Self: Sized,
	{
		OrderedSearchParameter(self, order)
	}
}

/// A resource that supports resolving a search parameter on it
pub trait Resolve: SearchableResource {
	/// Resolve a search parameter on this resource, returning the value of the
	/// first corresponding field found.
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord>;
}
