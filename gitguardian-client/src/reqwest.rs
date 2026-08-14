use gitguardian_api::{ApiCall, ApiConfig};
use http::Response;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Client {
    http: ::reqwest::Client,
    config: ApiConfig,
}

impl Client {
    pub fn new(config: ApiConfig) -> Self {
        Self::with_http(config, ::reqwest::Client::new())
    }

    pub fn with_http(config: ApiConfig, http: ::reqwest::Client) -> Self {
        Self { http, config }
    }

    pub fn config(&self) -> &ApiConfig {
        &self.config
    }

    pub async fn send<C: ApiCall>(&self, call: &C) -> Result<C::Output, Error> {
        let request = call.build(&self.config)?;
        let response = self
            .http
            .execute(::reqwest::Request::try_from(request)?)
            .await?;

        let status = response.status();
        let headers = response.headers().clone();
        let mut out = Response::new(response.bytes().await?);
        *out.status_mut() = status;
        *out.headers_mut() = headers;

        Ok(call.parse(out)?)
    }
}
