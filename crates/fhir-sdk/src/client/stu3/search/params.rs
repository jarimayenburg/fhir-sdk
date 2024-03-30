use crate::client::search::{escape_value, IntoQuery, SearchParameter};
use fhir_model::stu3::{
	codes::{SearchComparator, SearchModifierCode},
	resources::ResourceType,
};

/// Number search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone)]
pub struct NumberParam<'a> {
	/// How to compare the value
	pub comparator: Option<SearchComparator>,
	/// The numeric value
	pub value: &'a str,
}

impl<'a> SearchParameter for NumberParam<'a> {
	fn query_value(&self) -> String {
		let value = escape_value(self.value);

		if let Some(comparator) = self.comparator {
			format!("{}{}", comparator.as_ref(), value)
		} else {
			value
		}
	}
}

/// Date search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub struct DateParam<'a> {
	/// Search comparator to compare the date.
	pub comparator: Option<SearchComparator>,
	/// Value to search for.
	pub value: &'a str,
}

impl<'a> SearchParameter for DateParam<'a> {
	fn query_value(&self) -> String {
		let value = escape_value(self.value);

		if let Some(comparator) = self.comparator {
			format!("{}{}", comparator.as_ref(), value)
		} else {
			value
		}
	}
}

/// String search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum StringParam<'a> {
	/// Standard string search. This is a case-insensitive starts-with search.
	Standard(&'a str),
	/// Search a string that contains the given value.
	Contains(&'a str),
	/// Search a string that matches exactly the value, including casing and
	/// accents.
	Exact(&'a str),
}

impl<'a> SearchParameter for StringParam<'a> {
	fn modifier(&self) -> Option<&str> {
		match self {
			Self::Contains(_) => Some(SearchModifierCode::Contains.as_ref()),
			Self::Exact(_) => Some(SearchModifierCode::Exact.as_ref()),
			_ => None,
		}
	}

	fn query_value(&self) -> String {
		match self {
			Self::Standard(v) => v.to_string(),
			Self::Contains(v) => v.to_string(),
			Self::Exact(v) => v.to_string(),
		}
	}
}

/// Token search, e.g. in `CodeableConcept`s or `identifier`s.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum TokenParam<'a> {
	/// Standard token search (or `not` search).
	Standard {
		/// System for the value to search on. If this is given as empty string,
		/// the system must not be present.
		system: Option<&'a str>,
		/// Value of the code to search on.
		code: Option<&'a str>,
		/// Whether to switch to the `not` modifier.
		not: bool,
	},
	/// Token search whether the value is `in` or `not-in` a given `ValueSet`.
	In {
		/// `ValueSet` reference URI.
		value_set: &'a str,
		/// Whether to switch to `not-in` search.
		not: bool,
	},
	/// Tests the `text` or `display` values.
	Text {
		/// Text to search for (is a starts-with search).
		text: &'a str,
	},
}

impl<'a> SearchParameter for TokenParam<'a> {
	fn modifier(&self) -> Option<&str> {
		match self {
			TokenParam::Standard { not, .. } if *not => Some(SearchModifierCode::Not.as_ref()),
			TokenParam::In { not, .. } if *not => Some(SearchModifierCode::NotIn.as_ref()),
			TokenParam::In { .. } => Some(SearchModifierCode::In.as_ref()),
			TokenParam::Text { .. } => Some(SearchModifierCode::Text.as_ref()),
			_ => None,
		}
	}

	fn query_value(&self) -> String {
		match self {
			TokenParam::Standard { system, code, .. } => {
				if let Some(system) = system {
					format!("{}|{}", escape_value(system), escape_value(code.unwrap_or_default()))
				} else {
					escape_value(code.unwrap_or_default())
				}
			}
			TokenParam::In { value_set, .. } => escape_value(value_set),
			TokenParam::Text { text } => escape_value(text),
		}
	}
}

/// Search in references. Includes chaining, i.e. querying fields of the target
/// resource.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum ReferenceParam<'a> {
	/// Standard reference search by relative reference.
	Standard {
		/// Resource type of the resource.
		resource_type: ResourceType,
		/// ID of the resource the reference should point to.
		id: &'a str,
		/// Historic version id to search for.
		version_id: Option<&'a str>,
	},
	/// Standard reference search by absolute URL.
	Url {
		/// Reference URL.
		url: &'a str,
		/// Specific version id to search for.
		version_id: Option<&'a str>,
	},
}

impl<'a> SearchParameter for ReferenceParam<'a> {
	fn query_value(&self) -> String {
		match self {
			Self::Standard { resource_type, id, version_id: Some(version_id) } => {
				escape_value(&format!("{resource_type}/{id}/_history/{version_id}"))
			}
			Self::Standard { resource_type, id, version_id: None } => {
				escape_value(&format!("{resource_type}/{id}"))
			}
			Self::Url { url, version_id: Some(version_id) } => {
				format!("{}|{}", escape_value(url), escape_value(version_id))
			}
			Self::Url { url, version_id: None } => escape_value(url),
		}
	}
}

/// Search on a quantity.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub struct QuantityParam<'a> {
	/// Search comparator to compare the date.
	pub comparator: Option<SearchComparator>,
	/// Value to search for.
	pub value: &'a str,
	/// Optional system.
	pub system: Option<&'a str>,
	/// Optional code.
	pub code: Option<&'a str>,
}

impl<'a> SearchParameter for QuantityParam<'a> {
	fn query_value(&self) -> String {
		let value = if let Some(comparator) = self.comparator {
			format!("{}{}", comparator.as_ref(), escape_value(self.value))
		} else {
			escape_value(self.value)
		};

		if self.system.is_some() || self.code.is_some() {
			format!(
				"{value}|{}|{}",
				escape_value(self.system.unwrap_or_default()),
				escape_value(self.code.unwrap_or_default())
			)
		} else {
			value
		}
	}
}

/// Search on a URI.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum UriParam<'a> {
	/// Standard URI search, that matches exactly.
	Standard(&'a str),
	/// Match any URL that is below the given URL path, so contains more URL
	/// segments.
	Below(&'a str),
	/// Match any URL that is above the given URL path, so contains less URL
	/// segments.
	Above(&'a str),
}

impl<'a> SearchParameter for UriParam<'a> {
	fn modifier(&self) -> Option<&str> {
		match self {
			UriParam::Below(_) => Some(SearchModifierCode::Below.as_ref()),
			UriParam::Above(_) => Some(SearchModifierCode::Above.as_ref()),
			_ => None,
		}
	}

	fn query_value(&self) -> String {
		let uri = match self {
			Self::Standard(u) => u,
			Self::Below(u) => u,
			Self::Above(u) => u,
		};

		escape_value(uri)
	}
}

/// Search on any item whether it is a missing field using the `missing`
/// modifier.
#[derive(Debug, Clone, Copy)]
pub struct MissingParam(bool);

impl SearchParameter for MissingParam {
	fn modifier(&self) -> Option<&str> {
		Some(SearchModifierCode::Missing.as_ref())
	}

	fn query_value(&self) -> String {
		self.0.to_string()
	}
}

/// Include referred to resources in the search response
#[derive(Debug, Clone, Copy)]
pub struct IncludeParam<'a> {
	/// Resource type from which the join comes
	pub source_type: ResourceType,

	/// Field name to join on. Must be a search parameter of type reference for the [IncludeParam::source] resource type.
	pub field: &'a str,

	/// Type of the target resource in the case the reference field can have multiple target resource types.
	pub target_type: Option<ResourceType>,

	/// Whether to recursively include.
	pub recurse: bool,

	/// Whether this is a reverse include.
	pub reverse: bool,
}

impl<'a> IntoQuery for IncludeParam<'a> {
	fn into_query(self) -> (String, String) {
		let mut name: String = if self.reverse { "_revInclude" } else { "_include" }.to_string();

		if self.recurse {
			name += ":recurse";
		}

		let mut value = format!("{}:{}", self.source_type.as_str(), self.field);

		if let Some(target_type) = self.target_type {
			value = format!("{}:{}", value, target_type.as_str())
		}

		(name, value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn number() {
		let number = NumberParam { comparator: Some(SearchComparator::Lt), value: "60" };
		assert_eq!(number.query_value(), "lt60".to_owned());
	}

	#[test]
	fn token() {
		let token = TokenParam::Standard { system: None, code: Some("code"), not: true };
		assert_eq!(token.query_value(), "code".to_owned());

		let token = TokenParam::Standard { system: Some(""), code: Some("code"), not: false };
		assert_eq!(token.query_value(), "|code".to_owned());

		let token = TokenParam::Standard { system: Some("system"), code: None, not: false };
		assert_eq!(token.query_value(), "system|".to_owned());
	}

	#[test]
	fn quantity() {
		let quantity = QuantityParam { comparator: None, value: "1.0", system: None, code: None };
		assert_eq!(quantity.query_value(), "1.0".to_owned());

		let quantity =
			QuantityParam { comparator: None, value: "1.0", system: None, code: Some("g") };
		assert_eq!(quantity.query_value(), "1.0||g".to_owned());
	}

	#[test]
	fn missing() {
		let missing = MissingParam(true);
		assert_eq!(missing.query_value(), ("true".to_owned()));
	}

	#[test]
	fn include() {
		let include = IncludeParam {
			source_type: ResourceType::MedicationRequest,
			field: "encounter",
			target_type: None,
			recurse: false,
			reverse: false,
		};
		assert_eq!(
			include.into_query(),
			("_include".to_owned(), "MedicationRequest:encounter".to_owned())
		);

		let include = IncludeParam {
			source_type: ResourceType::Observation,
			field: "subject",
			target_type: Some(ResourceType::Patient),
			recurse: false,
			reverse: false,
		};
		assert_eq!(
			include.into_query(),
			("_include".to_owned(), "Observation:subject:Patient".to_owned())
		);

		let include = IncludeParam {
			source_type: ResourceType::Patient,
			field: "link",
			target_type: None,
			recurse: true,
			reverse: false,
		};
		assert_eq!(
			include.into_query(),
			("_include:recurse".to_owned(), "Patient:link".to_owned())
		);

		let include = IncludeParam {
			source_type: ResourceType::Encounter,
			field: "episode-of-care",
			target_type: None,
			recurse: false,
			reverse: true,
		};
		assert_eq!(
			include.into_query(),
			("_revInclude".to_owned(), "Encounter:episode-of-care".to_owned())
		);
	}
}
