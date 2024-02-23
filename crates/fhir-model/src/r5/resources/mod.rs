//! Resource definitions.

#[rustfmt::skip] // Too much for rustfmt
mod generated;
mod identifiable;

pub use generated::*;
pub use identifiable::*;

use crate::ParsedReference;

impl Bundle {
	/// Try to resolve an internal reference in the Bundle pretty much in accordance with
	/// the spec found at <https://www.hl7.org/fhir/r5/bundle.html#references>.
	///
	/// Deviations from the spec:
	/// 1. When resolving versioned absolute references, if multiple entries are found with matching fullUrl
	///    and with a resource with a matching version ID, the spec says to fail the lookup. Instead, we
	///    return the first one we find.
	/// 2. When resolving relative references, the spec says to use the root of the fullUrl of the entry
	///    of the resource that contains the reference field as the root to stick the relative reference
	///    on before comparing it to fullUrls of other entries. Instead, we always use the value of the
	///    `base_url` parameter as the root for relative references.
	///
	/// Conditional references and external reference resolution are unsupported
	pub fn resolve_reference(
		&self,
		base_url: &str,
		reference: &ParsedReference,
	) -> Option<&Resource> {
		let full_url = reference.to_string();

		match *reference {
			// If the reference is local it has to be looked up in the resource itself
			ParsedReference::Local { .. } => {
				panic!("Can't resolve local (contained) references in Bundle");
			}
			// If the reference is an URN, look for an entry with a matching fullUrl
			ParsedReference::Absolute { .. } if full_url.starts_with("urn:") => self
				.entry
				.iter()
				.flatten()
				.find(|e| e.full_url.as_ref() == Some(&full_url))
				.and_then(|e| e.resource.as_ref()),
			// If the reference is absolute and versionless, look for an entry with a matching fullUrl
			// If multiple are found, use the one with the most recent lastUpdated value.
			ParsedReference::Absolute { version_id: None, .. } => self
				.entry
				.iter()
				.flatten()
				.filter(|e| e.full_url.as_ref() == Some(&full_url))
				.filter_map(|e| e.resource.as_ref())
				.max_by_key(|r| {
					r.as_base_resource().meta().as_ref().and_then(|m| m.last_updated.as_ref())
				}),
			// If the reference is absolute and versioned, look for an entry with the versionless version
			// of the reference as its fullUrl and whose resource has a matching version ID.
			// We go slightly off spec here because we return the first resource found if there are multiple
			// resources in the Bundle with the same fullUrl and version ID.
			ParsedReference::Absolute { version_id: Some(version_id), .. } => self
				.entry
				.iter()
				.flatten()
				.filter(|e| e.full_url.as_ref() == Some(&full_url))
				.filter_map(|e| e.resource.as_ref())
				.find(|r| {
					r.as_base_resource().meta().as_ref().and_then(|m| m.version_id.as_ref())
						== Some(&version_id.to_string())
				}),
			// If the reference is relative, use the `base_url` parameter to make it
			// absolute and resolve that.
			ParsedReference::Relative { .. } => {
				self.resolve_reference(base_url, &reference.with_base_url(base_url))
			}
		}
	}
}
