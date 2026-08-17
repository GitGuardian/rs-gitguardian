use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use uuid::Uuid;

use crate::{
    ApiCall, ApiConfig,
    error::{ApiError, BuildError},
    models::api_token::ApiToken,
    util::json::expect_json,
};

/// Retrieve details of the current API token.
///
/// `GET /v1/api_tokens/self`, answering `200` Current token details or `401` Invalid API
/// key.
#[derive(Clone, Debug, Default)]
pub struct RetrieveCurrentApiToken;

impl ApiCall for RetrieveCurrentApiToken {
    type Output = ApiToken;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let url = config.endpoint("api_tokens/self")?;
        Ok(config.request(Method::GET, &url).body(Bytes::new())?)
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json(response, StatusCode::OK)
    }
}

/// Retrieve details of an API token.
///
/// `GET /v1/api_tokens/{token_id}`, answering `200` Current token details, `401` Invalid
/// API key or `404` API token not found.
#[derive(Clone, Debug)]
pub struct RetrieveApiToken {
    /// Id of the token.
    pub token_id: Uuid,
}

impl RetrieveApiToken {
    pub fn new(token_id: Uuid) -> Self {
        Self { token_id }
    }
}

impl ApiCall for RetrieveApiToken {
    type Output = ApiToken;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let url = config.endpoint(&format!("api_tokens/{}", self.token_id))?;
        Ok(config.request(Method::GET, &url).body(Bytes::new())?)
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json(response, StatusCode::OK)
    }
}
