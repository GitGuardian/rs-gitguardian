use serde::Deserialize;

/// Team of a GitGuardian workspace.
#[derive(Clone, Debug, Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    /// Team description.
    #[serde(default)]
    pub description: Option<String>,
    pub is_global: bool,
    pub gitguardian_url: String,
    #[serde(default)]
    pub external_provider_id: Option<String>,
}
