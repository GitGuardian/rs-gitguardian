use bytes::Bytes;
use http::{
    Request, Response, StatusCode,
    header::{CONTENT_TYPE, LINK},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::{ApiError, BuildError};
use crate::models::page::Page;
use crate::util::link::{Rel, cursor};

pub fn json_body<T: Serialize>(
    builder: http::request::Builder,
    value: &T,
) -> Result<Request<Bytes>, BuildError> {
    let body = serde_json::to_vec(value)?;
    Ok(builder
        .header(CONTENT_TYPE, "application/json")
        .body(Bytes::from(body))?)
}

pub fn expect_json<T: DeserializeOwned>(
    response: Response<Bytes>,
    expected: StatusCode,
) -> Result<T, ApiError> {
    let status = response.status();
    if status != expected {
        return Err(ApiError::Status {
            status,
            detail: String::from_utf8_lossy(response.body()).trim().to_owned(),
        });
    }
    serde_json::from_slice(response.body())
        .map_err(|source| ApiError::Deserialize { status, source })
}

/// Parses a JSON response returning paginated results
pub fn expect_json_page<T: DeserializeOwned>(
    response: Response<Bytes>,
    expected: StatusCode,
) -> Result<Page<T>, ApiError> {
    let link = response
        .headers()
        .get_all(LINK)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .collect::<Vec<_>>()
        .join(",");
    let items = expect_json(response, expected)?;
    Ok(Page {
        items,
        previous: cursor(&link, Rel::Prev),
        next: cursor(&link, Rel::Next),
    })
}
