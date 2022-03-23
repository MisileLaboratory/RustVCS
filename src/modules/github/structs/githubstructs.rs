use chrono::naive::NaiveDateTime;

pub struct GithubArtifact {
    id: u16,
    node_id: String,
    name: String,
    size_in_megabytes: u128,
    url: String,
    archive_download_url: String,
    expired: bool,
    created_at: NaiveDateTime,
    expires_at: NaiveDateTime,
    updated_at: NaiveDateTime
}