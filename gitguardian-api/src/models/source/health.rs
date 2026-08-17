use serde::{Deserialize, Serialize};

/// Health status of a source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SourceHealth {
    Safe,
    Unknown,
    AtRisk,
}
