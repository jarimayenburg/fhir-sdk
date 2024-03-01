//! FHIR Stu3 client implementation.

mod paging;
mod patch;
mod references;
mod response;
mod search;
mod transaction;

use fhir_model::{
	stu3::{
		resources::{
			BaseResource, Bundle, CapabilityStatement, DomainResource, NamedResource, Parameters,
			ParametersParameter, ParametersParameterValue, Patient, Resource, ResourceType,
		},
		types::Reference,
		JSON_MIME_TYPE,
	},
	ParsedReference,
};
use futures::Stream;
use reqwest::{
	header::{self, HeaderValue},
	StatusCode, Url,
};
use serde::Serialize;

pub use self::search::{
	DateSearch, MissingSearch, NumberSearch, QuantitySearch, ReferenceSearch, StringSearch,
	TokenSearch, UriSearch,
};
use self::{
	paging::Paged,
	patch::{PatchViaFhir, PatchViaJson},
	transaction::BatchTransaction,
};
use super::{misc, Client, Error, FhirStu3, SearchParameters};

/// FHIR MIME-type this client uses.
const MIME_TYPE: &str = JSON_MIME_TYPE;

impl Client<FhirStu3> {
	/// Get the server's capabilities. Fails if the respective FHIR version is
	/// not supported at all.
	pub async fn capabilities(&self) -> Result<CapabilityStatement, Error> {
		let url = self.url(&["metadata"]);
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;

		response.body().await
	}

	/// Read any resource from any URL.
	async fn read_generic<R: TryFrom<Resource>>(&self, url: Url) -> Result<Option<R>, Error> {
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;

		if [StatusCode::NOT_FOUND, StatusCode::GONE].contains(&response.status()) {
			return Ok(None);
		}

		response.body().await.map(Some)
	}

	/// Read the current version of a specific FHIR resource.
	pub async fn read<R: NamedResource + TryFrom<Resource>>(
		&self,
		id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE.as_str(), id]);
		self.read_generic(url).await
	}

	/// Read a specific version of a specific FHIR resource.
	pub async fn read_version<R: NamedResource + TryFrom<Resource>>(
		&self,
		id: &str,
		version_id: &str,
	) -> Result<Option<R>, Error> {
		let url = self.url(&[R::TYPE.as_str(), id, "_history", version_id]);
		self.read_generic(url).await
	}

	/// Read the resource that is targeted in the reference.
	pub async fn read_referenced(&self, reference: &Reference) -> Result<Resource, Error> {
		let parsed_reference = reference.parse().ok_or(Error::MissingReference)?;

		let absolute: String = match parsed_reference {
			ParsedReference::Local { .. } => return Err(Error::LocalReference),
			ParsedReference::Relative { .. } => {
				parsed_reference.with_base_url(self.0.base_url.as_str()).to_string()
			}
			absolute => absolute.to_string(),
		};

		let url: Url = absolute.parse().map_err(|_| Error::UrlParse(absolute))?;

		let resource: Resource = self
			.read_generic(url.clone())
			.await?
			.ok_or_else(|| Error::ResourceNotFound(url.to_string()))?;

		Ok(resource)
	}

	/// Create a new FHIR resource on the FHIR server. Returns the resource ID
	/// and version ID.
	pub async fn create<R: NamedResource + Serialize + Send + Sync>(
		&self,
		resource: &R,
	) -> Result<(String, Option<String>), Error> {
		let url = self.url(&[R::TYPE.as_str()]);
		let request = self
			.0
			.client
			.post(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, MIME_TYPE)
			.json(resource);

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let (id, version_id) = misc::parse_location(response.headers())?;
			let version_id = version_id.or(misc::parse_etag(response.headers()).ok());
			Ok((id, version_id))
		} else {
			Err(response.successful().await.unwrap_err())
		}
	}

	/// Update a FHIR resource (or create it if it did not
	/// exist). If conditional update is selected, the resource is only updated
	/// if the version ID matches the expectations.
	pub async fn update<R: NamedResource + BaseResource + Serialize + Send + Sync>(
		&self,
		resource: &R,
		conditional: bool,
	) -> Result<(bool, String), Error> {
		let id = resource.id().as_ref().ok_or(Error::MissingId)?;

		let url = self.url(&[R::TYPE.as_str(), id]);
		let mut request = self
			.0
			.client
			.put(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, MIME_TYPE)
			.json(resource);
		if conditional {
			let version_id = resource
				.meta()
				.as_ref()
				.and_then(|meta| meta.version_id.as_ref())
				.ok_or(Error::MissingVersionId)?;
			let if_match = HeaderValue::from_str(&format!("W/\"{version_id}\""))
				.map_err(|_| Error::MissingVersionId)?;
			request = request.header(header::IF_MATCH, if_match);
		}

		let response = self.run_request(request).await?;
		if response.status().is_success() {
			let created = response.status() == StatusCode::CREATED;
			let version_id = misc::parse_etag(response.headers())?;
			Ok((created, version_id))
		} else {
			Err(response.successful().await.unwrap_err())
		}
	}

	/// Begin building a patch request for a FHIR resource on the server via the
	/// `FHIRPath Patch` method.
	pub fn patch_via_fhir<'a>(&self, resource_type: ResourceType, id: &'a str) -> PatchViaFhir<'a> {
		PatchViaFhir::new(self.clone(), resource_type, id)
	}

	/// Begin building a patch request for a FHIR resource on the server via the
	/// [`JSON Patch`](https://datatracker.ietf.org/doc/html/rfc6902) method.
	pub fn patch_via_json<'a>(&self, resource_type: ResourceType, id: &'a str) -> PatchViaJson<'a> {
		PatchViaJson::new(self.clone(), resource_type, id)
	}

	/// Delete a FHIR resource on the server.
	pub async fn delete(&self, resource_type: ResourceType, id: &str) -> Result<(), Error> {
		let url = self.url(&[resource_type.as_str(), id]);
		let request = self.0.client.delete(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;

		response.successful().await
	}

	/// Search for FHIR resources of a given type given the query parameters.
	/// This simply ignores resources of the wrong type, e.g. an additional
	/// OperationOutcome.
	pub fn search<R>(
		&self,
		queries: SearchParameters,
	) -> impl Stream<Item = Result<R, Error>> + Send + 'static
	where
		R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
	{
		let mut url = self.url(&[R::TYPE.as_str()]);
		url.query_pairs_mut().extend_pairs(queries.into_queries()).finish();

		Paged::new(self.clone(), url)
	}

	/// Start building a new batch request.
	pub fn batch(&self) -> BatchTransaction {
		BatchTransaction::new(self.clone(), false)
	}

	/// Start building a new transaction request.
	pub fn transaction(&self) -> BatchTransaction {
		BatchTransaction::new(self.clone(), true)
	}

	/// Operation `$everything` on `Encounter`, returning a Bundle with all
	/// resources for an `Encounter` record.
	pub async fn operation_encounter_everything(&self, id: &str) -> Result<Bundle, Error> {
		let url = self.url(&["Encounter", id, "$everything"]);
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;

		response.body().await
	}

	/// Operation `$everything` on `Patient`, returning a Bundle with all
	/// resources for an `Patient` record.
	pub async fn operation_patient_everything(&self, id: &str) -> Result<Bundle, Error> {
		let url = self.url(&["Patient", id, "$everything"]);
		let request = self.0.client.get(url).header(header::ACCEPT, MIME_TYPE);

		let response = self.run_request(request).await?;

		response.body().await
	}

	/// Operation `$match` on `Patient`, returning matches for Patient records
	/// based on a given incomplete Patient resource.
	pub async fn operation_patient_match(
		&self,
		patient: Patient,
		only_certain: bool,
		count: i32,
	) -> Result<Bundle, Error> {
		#[allow(clippy::unwrap_used)] // Will always succeed.
		let parameters = Parameters::builder()
			.parameter(vec![
				Some(
					ParametersParameter::builder()
						.name("resource".to_owned())
						.resource(Resource::from(patient))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("onlyCertainMatches".to_owned())
						.value(ParametersParameterValue::Boolean(only_certain))
						.build()
						.unwrap(),
				),
				Some(
					ParametersParameter::builder()
						.name("count".to_owned())
						.value(ParametersParameterValue::Integer(count))
						.build()
						.unwrap(),
				),
			])
			.build()
			.unwrap();

		let url = self.url(&["Patient", "$match"]);
		let request = self
			.0
			.client
			.post(url)
			.header(header::ACCEPT, MIME_TYPE)
			.header(header::CONTENT_TYPE, MIME_TYPE)
			.json(&parameters);

		let response = self.run_request(request).await?;

		response.body().await
	}
}
