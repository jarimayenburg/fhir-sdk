use fhir_model::r5::resources::{Bundle, DomainResource, TypedResource};
use fhir_model::ParsedReference;

use super::{Client, FhirR5};

/// Looks up all references in a resource and populates reference target fields with any matching
/// resources it can find in the Bundle.
pub fn populate_reference_targets<R: DomainResource>(
	client: &Client<FhirR5>,
	bundle: &Bundle,
	resource: &mut R,
) {
	let resource_type_and_id = format!(
		"{}/{}",
		resource.resource_type(),
		resource.id().clone().unwrap_or("<unknown>".to_string())
	);

	let contained = resource.contained().clone();

	for field in resource.lookup_references() {
		if let Some(reference) = field.reference().clone().parse() {
			let target = match reference {
				ParsedReference::Local { id } => {
					contained.iter().find(|c| c.as_base_resource().id() == &Some(id.to_string()))
				}
				other => bundle.resolve_reference(client.0.base_url.as_str(), &other),
			};

			if let Some(target) = target {
				if field.set_target(target.clone()).is_err() {
					tracing::warn!("Reference {} in Bundled resource {} refers to resource of unsupported type {}", reference.to_string(), resource_type_and_id, target.resource_type());
				}
			} else {
				tracing::debug!(
					"Unable to resolve reference {} in Bundled resource {}",
					reference.to_string(),
					resource_type_and_id,
				);
			}
		}
	}
}
