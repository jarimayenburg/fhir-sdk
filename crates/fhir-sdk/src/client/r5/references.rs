use fhir_model::r5::resources::{Bundle, DomainResource, TypedResource};
use fhir_model::ParsedReference;

/// Looks up all references in a resource and populates reference target fields with any matching
/// resources it can find in the Bundle.
pub fn populate_reference_targets<R: DomainResource>(
	base_url: &str,
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
				other => bundle.resolve_reference(base_url, &other),
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

#[cfg(test)]
mod tests {
	use super::*;

	use fhir_model::r5::{
		codes::{BundleType, ObservationStatus},
		local_reference_to, reference_to,
		resources::{
			Bundle, BundleEntry, Observation, ObservationSubjectReferenceTarget, Patient, Resource,
		},
		types::{CodeableConcept, Reference},
	};
	use uuid::Uuid;

	const BASE_URL: &'static str = "http://my.fhir.server/fhir";

	#[test]
	fn populates_references_local() {
		let patient = Patient::builder().id(Uuid::new_v4().to_string()).build().unwrap();

		let subject = local_reference_to(&patient).unwrap();

		let mut observation = test_observation();
		observation.contained = vec![patient.clone().into()];
		observation.subject = Some(subject.into());

		let bundle = test_bundle(vec![]);

		populate_reference_targets(BASE_URL, &bundle, &mut observation);

		assert_eq!(observation.contained, vec![Resource::Patient(patient)]);
	}

	#[test]
	fn populates_references_urn() {
		let patient_urn = Uuid::new_v4().urn().to_string();
		let subject = Reference::builder().reference(patient_urn.clone()).build().unwrap();

		let patient = test_patient();

		let mut observation = test_observation();
		observation.subject = Some(subject.into());

		let observation_ref = reference_to(&observation).unwrap();
		let observation_full_url =
			observation_ref.parse().unwrap().with_base_url(BASE_URL).to_string();

		let bundle = test_bundle(vec![
			(&patient_urn, patient.clone().into()),
			(&observation_full_url, observation.clone().into()),
		]);

		populate_reference_targets(BASE_URL, &bundle, &mut observation);

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	#[test]
	fn populates_references_absolute() {
		let patient = test_patient();
		let patient_ref = reference_to(&patient).unwrap();
		let patient_full_url = patient_ref.parse().unwrap().with_base_url(BASE_URL);

		let mut observation = test_observation();
		observation.subject = Some(Reference::from(patient_full_url).into());

		let observation_ref = reference_to(&observation).unwrap();
		let observation_full_url = observation_ref.parse().unwrap().with_base_url(BASE_URL);

		let bundle = test_bundle(vec![
			(&patient_full_url.to_string(), patient.clone().into()),
			(&observation_full_url.to_string(), observation.clone().into()),
		]);

		populate_reference_targets(BASE_URL, &bundle, &mut observation);

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	#[test]
	fn populates_references_relative() {
		let patient = test_patient();
		let patient_ref = reference_to(&patient).unwrap();
		let patient_full_url = patient_ref.parse().unwrap().with_base_url(BASE_URL);

		let mut observation = test_observation();
		observation.subject = Some(Reference::from(patient_ref.clone()).into());

		let observation_ref = reference_to(&observation).unwrap();
		let observation_full_url = observation_ref.parse().unwrap().with_base_url(BASE_URL);

		let bundle = test_bundle(vec![
			(&patient_full_url.to_string(), patient.clone().into()),
			(&observation_full_url.to_string(), observation.clone().into()),
		]);

		populate_reference_targets(BASE_URL, &bundle, &mut observation);

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	fn test_bundle(resources: Vec<(&str, Resource)>) -> Bundle {
		let mut bundle = Bundle::builder().r#type(BundleType::Collection).build().unwrap();

		for (full_url, resource) in resources {
			let entry = BundleEntry::builder()
				.full_url(full_url.to_string())
				.resource(resource)
				.build()
				.unwrap();

			bundle.entry.push(Some(entry));
		}

		bundle
	}

	fn test_patient() -> Patient {
		Patient::builder().id(Uuid::new_v4().to_string()).build().unwrap()
	}

	fn test_observation() -> Observation {
		Observation::builder()
			.id(Uuid::new_v4().to_string())
			.status(ObservationStatus::Final)
			.code(CodeableConcept::builder().build().unwrap())
			.build()
			.unwrap()
	}
}
