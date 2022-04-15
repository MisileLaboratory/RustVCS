use chrono::naive::NaiveDateTime;

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
