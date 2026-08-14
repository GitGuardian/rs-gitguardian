use serde::Deserialize;
use uuid::Uuid;

/// Honeytoken inserted into a realistic file.
#[derive(Clone, Debug, Deserialize)]
pub struct HoneytokenWithContext {
    pub content: String,
    pub filename: String,
    pub language: String,
    pub suggested_commit_message: String,
    pub honeytoken_id: Uuid,
    /// URL of the honeytoken on the dashboard.
    pub gitguardian_url: String,
}
