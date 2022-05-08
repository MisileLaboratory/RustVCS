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

pub struct GithubArtifacts {
    pub total_count: u128,
    pub artifacts: Vec<GithubArtifact>
}

#[derive(Deserialize, Serialize)]
pub struct GithubCacheUsage {
    pub total_active_caches_size_in_bytes: u128,
    pub total_active_caches_count: u128
}

#[derive(Deserialize, Serialize)]
pub struct GithubProjectCacheUsage {
    pub full_name: String,
    pub active_caches_size_in_bytes: u128,
    pub active_caches_count: u16
}
