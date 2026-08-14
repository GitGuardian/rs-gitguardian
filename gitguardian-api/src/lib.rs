pub mod api;
pub mod constants;
pub mod error;
pub mod models;
pub(crate) mod util;

pub use bytes;
pub use chrono;
pub use http;
pub use uuid;

use bytes::Bytes;
use http::header::{AUTHORIZATION, HeaderValue};
use http::{Method, Request, Response};
use url::Url;

use crate::constants::DEFAULT_BASE_URI;
use crate::error::{ApiError, BuildError, ConfigError};

pub trait ApiCall {
    type Output;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError>;

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError>;
}

/// Base URI and API key every call is built against.
///
/// The GitGuardian API uses API keys to authenticate requests, sent as
/// `Token <api-key>` in the `Authorization` header.
#[derive(Clone, Debug)]
pub struct ApiConfig {
    base: Url,
    authorization: HeaderValue,
}

impl ApiConfig {
    pub fn new(api_key: &str) -> Result<Self, ConfigError> {
        Self::with_base_uri(api_key, DEFAULT_BASE_URI)
    }

    pub fn with_base_uri(api_key: &str, base_uri: &str) -> Result<Self, ConfigError> {
        let mut authorization =
            HeaderValue::from_str(&format!("Token {api_key}")).map_err(|_| ConfigError::ApiKey)?;
        authorization.set_sensitive(true);
        Ok(Self {
            base: Url::parse(base_uri)?,
            authorization,
        })
    }

    pub fn request(&self, method: Method, url: &Url) -> http::request::Builder {
        Request::builder()
            .method(method)
            .uri(url.as_str())
            .header(AUTHORIZATION, self.authorization.clone())
    }

    pub fn endpoint(&self, path: &str) -> Result<Url, BuildError> {
        Ok(self.base.join(path)?)
    }
}
