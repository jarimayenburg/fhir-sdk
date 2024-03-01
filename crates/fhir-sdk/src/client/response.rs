use std::ops::Deref;

use async_trait::async_trait;
use reqwest::Url;
use serde::de::DeserializeOwned;

use super::Error;

pub struct FhirResponse {
	pub(super) base_url: Url,
	pub(super) response: reqwest::Response,
}

#[async_trait]
pub(crate) trait ParseResponseBody<R: DeserializeOwned> {
	/// Attempts to parse the response body as a FHIR resource.
	/// Returns an [Error] if the request failed or the body could not be
	/// parsed as `R`.
	async fn body(self) -> Result<R, Error>;

	/// Attempts to parse the response body as a FHIR resource, without
	/// checking if the request is successful. That means that for failed
	/// requests, `R` (in most cases) has to be an OperationOutcome for
	/// this method to succeed. Returns an [Error] if the request could
	/// not be parsed as `R`.
	///
	/// You'll likely want to use [ParseResponseBody::body] instead.
	async fn body_unchecked(self) -> Result<R, Error>;
}

impl FhirResponse {
	pub fn new(base_url: Url, response: reqwest::Response) -> Self {
		FhirResponse { base_url, response }
	}
}

impl Deref for FhirResponse {
	type Target = reqwest::Response;

	fn deref(&self) -> &Self::Target {
		&self.response
	}
}
