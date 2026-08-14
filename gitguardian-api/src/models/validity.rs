use serde::Deserialize;

/// Validity of the found secret.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Validity {
    Valid,
    Invalid,
    Unknown,
    FailedToCheck,
    NoChecker,
}
