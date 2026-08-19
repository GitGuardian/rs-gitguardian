use serde::Deserialize;

/// Whether the client should write or delete the placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeploymentAction {
    Write,
    Delete,
}
