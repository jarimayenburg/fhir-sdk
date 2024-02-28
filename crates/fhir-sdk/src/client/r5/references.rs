use fhir_model::r5::resources::{Bundle, DomainResource, Resource, TypedResource};
use fhir_model::ParsedReference;

use super::{Client, FhirR5};

impl Client<FhirR5> {
	/// Looks up all references in `resource` and populates reference target fields with
	/// any matching resources it can find in the contained resource or in the Bundle.
	pub(super) fn populate_reference_targets_bundle(&self, bundle: &mut Bundle) {
		let lookup = bundle.clone();

		for entry in bundle.entry.iter_mut().flatten() {
			if let Some(resource) = entry.resource.as_mut() {
				if let Some(domain_resource) = resource.as_domain_resource_mut() {
					self.populate_reference_targets_internal(domain_resource, Some(&lookup));
				}
			}
		}
	}

	pub(super) fn populate_reference_targets<R: DomainResource>(&self, resource: &mut R) {
		self.populate_reference_targets_internal(resource, None);
	}

	pub(super) fn populate_reference_targets_resource(&self, resource: &mut Resource) {
		if let Resource::Bundle(bundle) = resource {
			self.populate_reference_targets_bundle(bundle)
		} else if let Some(domain_resource) = resource.as_domain_resource_mut() {
			self.populate_reference_targets_internal(domain_resource, None)
		}
	}

	pub(super) fn populate_reference_targets_internal(
		&self,
		resource: &mut dyn DomainResource,
		bundle: Option<&Bundle>,
	) {
		let contained = resource.contained().clone();

		for field in resource.lookup_references() {
			if let Some(reference) = field.reference().clone().parse() {
				let target = match reference {
					ParsedReference::Local { id } => contained
						.iter()
						.find(|c| c.as_base_resource().id() == &Some(id.to_string())),
					other if bundle.is_some() => {
						bundle.unwrap().resolve_reference(self.0.base_url.as_str(), &other)
					}
					_ => None,
				};

				if let Some(target) = target {
					if field.set_target(target.clone()).is_err() {
						tracing::warn!("Reference {} in Bundled resource refers to resource of unsupported type {}", reference.to_string(), target.resource_type());
					}
				} else {
					tracing::debug!(
						"Unable to resolve reference {} in Bundled resource",
						reference.to_string(),
					);
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use fhir_model::r5::{
		codes::{BundleType, ObservationStatus},
		local_reference_to, reference_to,
		resources::{
			Bundle, BundleEntry, Observation, ObservationSubjectReferenceTarget, Patient, Resource,
		},
		types::{CodeableConcept, Reference},
	};
	use uuid::Uuid;

	use crate::{
		client::{Client, FhirR5},
		Url,
	};

	const BASE_URL: &'static str = "http://my.fhir.server/fhir";

	#[test]
	fn populates_references_local() {
		let client = client();

		let patient = Patient::builder().id(Uuid::new_v4().to_string()).build().unwrap();

		let subject = local_reference_to(&patient).unwrap();

		let mut observation = test_observation();
		observation.contained = vec![patient.clone().into()];
		observation.subject = Some(subject.into());

		client.populate_reference_targets(&mut observation);

		assert_eq!(observation.contained, vec![Resource::Patient(patient)]);
	}

	#[test]
	fn populates_references_urn() {
		let client = client();

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

		client.populate_reference_targets_internal(&mut observation, Some(&bundle));

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	#[test]
	fn populates_references_absolute() {
		let client = client();

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

		client.populate_reference_targets_internal(&mut observation, Some(&bundle));

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	#[test]
	fn populates_references_relative() {
		let client = client();

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

		client.populate_reference_targets_internal(&mut observation, Some(&bundle));

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	fn client() -> Client<FhirR5> {
		Client::new(Url::parse(BASE_URL).unwrap()).unwrap()
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
