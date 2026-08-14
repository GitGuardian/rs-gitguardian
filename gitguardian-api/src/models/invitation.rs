use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::models::access_level::AccessLevel;

/// Invitation sent to a user of a GitGuardian workspace.
#[derive(Clone, Debug, Deserialize)]
pub struct Invitation {
    pub id: u32,
    pub email: String,
    pub access_level: AccessLevel,
    pub date: DateTime<Utc>,
}
