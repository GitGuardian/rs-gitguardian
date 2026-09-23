use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};

use crate::{
    ApiCall, ApiConfig,
    error::{ApiError, BuildError},
    models::custom_host::DetectorCustomHostConfig,
    util::json::expect_json,
};

#[derive(Clone, Debug, Default)]
pub struct ListDetectorCustomHosts;

impl ApiCall for ListDetectorCustomHosts {
    type Output = Vec<DetectorCustomHostConfig>;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let url = config.endpoint("detector-custom-hosts")?;
        Ok(config.request(Method::GET, &url).body(Bytes::new())?)
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json(response, StatusCode::OK)
    }
}
