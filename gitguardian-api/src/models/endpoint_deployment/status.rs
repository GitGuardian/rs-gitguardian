use serde::{Deserialize, Serialize};

/// Outcome reported by the client.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeploymentStatus {
    Pending,
    Planted,
    Failed,
    Removed,
}
