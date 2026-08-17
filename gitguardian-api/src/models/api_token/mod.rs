use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::api_token::{
    scope::ApiTokenScope, status::ApiTokenStatus, r#type::ApiTokenType,
};

pub mod scope;
pub mod status;
pub mod r#type;

/// Details of an API Token.
#[derive(Clone, Debug, Deserialize)]
pub struct ApiToken {
    /// Id of API token.
    pub id: Uuid,
    /// Name of API token.
    pub name: String,
    /// Id of the workspace this token belongs to.
    pub workspace_id: u32,
    #[serde(rename = "type")]
    pub token_type: ApiTokenType,
    pub status: ApiTokenStatus,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub last_used_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub expire_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub revoked_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub member_id: Option<u32>,
    #[serde(default)]
    pub creator_id: Option<u32>,
    #[serde(default)]
    pub scopes: Vec<ApiTokenScope>,
}
