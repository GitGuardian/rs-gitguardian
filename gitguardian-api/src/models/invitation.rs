use serde::Deserialize;

/// Invitation sent to a user of a GitGuardian workspace.
#[derive(Clone, Debug, Deserialize)]
pub struct Invitation {
    pub id: u32,
    pub email: String,
    pub access_level: String,
    pub date: String,
}
