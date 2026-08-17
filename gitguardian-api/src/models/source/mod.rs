use serde::Deserialize;

use crate::models::source::{
    health::SourceHealth, last_scan::LastScan, monitoring_status::MonitoringStatus,
    provider_metadata::ProviderMetadata, secret_incidents_breakdown::SecretIncidentsBreakdown,
};

pub mod criticality;
pub mod health;
pub mod last_scan;
pub mod monitoring_status;
pub mod ordering;
pub mod provider_metadata;
pub mod scan_status;
pub mod secret_incidents_breakdown;
pub mod r#type;
pub mod visibility;

/// Source known by GitGuardian.
#[derive(Clone, Debug, Deserialize)]
pub struct Source {
    pub id: u32,
    pub url: String,
    #[serde(rename = "type")]
    pub source_type: String,
    pub full_name: String,
    pub health: SourceHealth,
    #[serde(default)]
    pub default_branch: Option<String>,
    #[serde(default)]
    pub default_branch_head: Option<String>,
    pub open_incidents_count: u32,
    pub closed_incidents_count: u32,
    pub secret_incidents_breakdown: SecretIncidentsBreakdown,
    pub visibility: String,
    pub external_id: String,
    /// Criticality of the source.
    pub source_criticality: String,
    #[serde(default)]
    pub last_scan: Option<LastScan>,
    pub monitored: bool,
    pub monitoring_status: MonitoringStatus,
    pub provider_metadata: ProviderMetadata,
    pub deleted: bool,
}
