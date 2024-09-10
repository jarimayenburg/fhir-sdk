/// A resource type that can be searched on
pub trait ResourceWithSearchParameters {
	/// Enum of parameters that can be used in the search
	type Params;
}
