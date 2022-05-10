use chrono::naive::NaiveDateTime;

use serde::{Deserialize, Serialize};

pub struct GithubArtifact {
    pub id: u128,
    pub node_id: String,
    pub name: String,
    pub size_in_megabytes: u128,
    pub url: String,
    pub archive_download_url: String,
    pub expired: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Deserialize, Serialize)]
pub struct GithubCacheUsage {
    #[serde(rename = "total_active_caches_size_in_bytes")]
    pub size_in_bytes: u128,
    #[serde(rename = "total_active_caches_count")]
    pub count: u128
}

#[derive(Deserialize, Serialize)]
pub struct GithubProjectCacheUsage {
    pub full_name: String,
    #[serde(rename = "total_active_caches_size_in_bytes")]
    pub size_in_bytes: u128,
    #[serde(rename = "total_active_caches_count")]
    pub count: u16
}

#[derive(Deserialize, Serialize)]
pub struct GithubActionsPermissions {
    pub enabled_organizations: String,
    pub allowed_actions: String,
    #[serde(rename = "selected_actions_url")]
    pub url: String
}