use std::marker::PhantomData;
use std::ops::Deref;

use reqwest::Url;

pub struct FhirResponse<V> {
	pub(super) base_url: Url,
	pub(super) response: reqwest::Response,
	version: PhantomData<V>,
}

impl<V> FhirResponse<V> {
	pub fn new(base_url: Url, response: reqwest::Response) -> Self {
		FhirResponse { base_url, response, version: PhantomData::default() }
	}
}

impl<V> Deref for FhirResponse<V> {
	type Target = reqwest::Response;

	fn deref(&self) -> &Self::Target {
		&self.response
	}
}
