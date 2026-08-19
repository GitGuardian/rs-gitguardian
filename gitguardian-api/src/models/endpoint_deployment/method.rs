use serde::{Deserialize, Serialize};

/// Placement method. Selects how the honeytoken is materialized on the endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeploymentMethod {
    AwsCredentials,
    AwsConfigProfile,
}
