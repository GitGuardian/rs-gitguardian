use serde::Deserialize;

/// Access level of a member on a workspace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AccessLevel {
    Owner,
    Manager,
    Member,
    Restricted,
}
