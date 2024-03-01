use std::ops::Deref;

use reqwest::Url;

pub struct FhirResponse {
	pub(super) base_url: Url,
	pub(super) response: reqwest::Response,
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
