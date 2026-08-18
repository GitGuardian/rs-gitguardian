use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use serde::Serialize;

use crate::{
    ApiCall, ApiConfig,
    error::{ApiError, BuildError},
    models::{document::Document, scan_result::ScanResult},
    util::{
        json::{expect_json, json_body},
        url::set_query,
    },
};

/// Query parameters for a scan request
#[derive(Clone, Debug, Serialize)]
struct ScanQueryParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_secrets: Option<bool>,
}

/// Content scan.
///
/// Scan provided document content for policy breaks.
///
/// Request body shouldn't exceed 1MB.
///
/// This endpoint is stateless and as such stores neither the documents nor the secrets
/// found on GitGuardian's servers.
///
/// `POST /v1/scan`, answering `200` Successful Scan, `400` Invalid data, `401` Invalid
/// API key, `403` Quota limit reached or `503` API under maintenance.
#[derive(Clone, Debug)]
pub struct Scan {
    pub document: Document,
    /// Indicates whether all secrets should be returned.
    pub all_secrets: Option<bool>,
}

impl Scan {
    pub fn new(document: Document) -> Self {
        Self {
            document,
            all_secrets: None,
        }
    }
}

impl From<Document> for Scan {
    fn from(document: Document) -> Self {
        Self::new(document)
    }
}

impl ApiCall for Scan {
    type Output = ScanResult;

    fn build(&self, config: &ApiConfig) -> Result<Request<Bytes>, BuildError> {
        let mut url = config.endpoint("scan")?;
        set_query(
            &mut url,
            &ScanQueryParam {
                all_secrets: self.all_secrets,
            },
        )?;
        json_body(config.request(Method::POST, &url), &self.document)
    }

    fn parse(&self, response: Response<Bytes>) -> Result<Self::Output, ApiError> {
        expect_json(response, StatusCode::OK)
    }
}
