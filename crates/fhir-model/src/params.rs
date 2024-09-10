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
}
