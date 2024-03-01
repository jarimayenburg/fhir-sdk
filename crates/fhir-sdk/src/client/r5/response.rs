use async_trait::async_trait;
use fhir_model::r5::resources::{OperationOutcome, Resource, TypedResource};
use reqwest::Url;

use crate::client::response::{FhirResponse, ParseResponseBody};
use crate::client::Error;

use super::references::populate_reference_targets_resource;

#[async_trait]
impl<R: TryFrom<Resource>> ParseResponseBody<R> for FhirResponse {
	async fn body(self) -> Result<R, Error> {
		let status = self.status();
		let body = self.response.text().await?;

		if status.is_success() {
			parse(&self.base_url, &body)
		} else if let Ok(outcome) = parse::<OperationOutcome>(&self.base_url, &body) {
			Err(Error::OperationOutcomeR5(status, outcome))
		} else {
			Err(Error::Response(status, body))
		}
	}

	async fn body_unchecked(self) -> Result<R, Error> {
		let body = self.response.text().await?;

		parse(&self.base_url, &body)
	}
}

impl FhirResponse {
	pub async fn successful(self) -> Result<(), Error> {
		let status = self.response.status();

		if status.is_success() {
			return Ok(());
		}

		let body = &self.response.text().await.unwrap_or_default();
		if let Ok(outcome) = parse::<OperationOutcome>(&self.base_url, body) {
			Err(Error::OperationOutcomeR5(status, outcome))
		} else {
			Err(Error::Response(status, body.to_string()))
		}
	}
}

fn parse<R: TryFrom<Resource>>(base_url: &Url, body: &str) -> Result<R, Error> {
	let mut resource: Resource = serde_json::from_str(body)?;
	let resource_type = resource.resource_type();

	populate_reference_targets_resource(base_url, &mut resource);

	resource.try_into().map_err(|_| Error::UnexpectedResourceType(resource_type.to_string()))
}
