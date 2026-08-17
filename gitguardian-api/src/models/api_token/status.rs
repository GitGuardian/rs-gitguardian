use serde::Deserialize;

/// Status of an API token.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ApiTokenStatus {
    Active,
    Expired,
    Revoked,
}
