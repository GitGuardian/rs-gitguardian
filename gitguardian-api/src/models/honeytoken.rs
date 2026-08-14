use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::custom_tag::CustomTag;
use crate::models::honeytoken_status::HoneytokenStatus;
use crate::models::honeytoken_tag::HoneytokenTag;
use crate::models::honeytoken_type::HoneytokenType;

/// Honeytoken of a GitGuardian workspace.
#[derive(Clone, Debug, Deserialize)]
pub struct Honeytoken {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub gitguardian_url: String,
    pub status: HoneytokenStatus,
    #[serde(default)]
    pub triggered_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub revoked_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub open_events_count: Option<u32>,
    #[serde(rename = "type")]
    pub honeytoken_type: HoneytokenType,
    #[serde(default)]
    pub creator_id: Option<u32>,
    #[serde(default)]
    pub revoker_id: Option<u32>,
    #[serde(default)]
    pub creator_api_token_id: Option<Uuid>,
    #[serde(default)]
    pub revoker_api_token_id: Option<Uuid>,
    /// Secret to be placed as a honeytoken.
    #[serde(default)]
    pub token: HashMap<String, String>,
    #[serde(default)]
    pub tags: Vec<HoneytokenTag>,
    /// Custom tags set on the honeytoken.
    #[serde(default)]
    pub custom_tags: Vec<CustomTag>,
}
