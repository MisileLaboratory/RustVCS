use crate::modules::base::requesthandler::get_default_headers;
use crate::modules::github::structs::githubstructs::*;

use chrono::format::ParseResult;
use chrono::NaiveDateTime;

use reqwest::{Client, Error};

use async_trait::async_trait;

use serde::Deserialize;

pub struct GithubHandler {
    client: Client,
    base_url: String
}

#[derive(Deserialize, Clone)]
struct TempGithubArtifact {
    pub id: u128,
    pub node_id: String,
    pub name: String,
    pub size_in_megabytes: u128,
    pub url: String,
    pub archive_download_url: String,
    pub expired: bool,
    pub created_at: String,
    pub expires_at: String,
    pub updated_at: String
}

#[derive(Deserialize)]
struct TempGithubArtifacts {
    pub total_count: u128,
    pub artifacts: Vec<TempGithubArtifact>
}

#[async_trait]
pub trait GithubHandlingTrait {
    fn new(access_token: String) -> Self;
    fn github_time_parse(&self, timestring: String) -> ParseResult<NaiveDateTime>;
    async fn get_list_of_artifacts(&self, owner: String, repo: String) -> Result<Option<GithubArtifacts>, Error>;
    async fn get_artifact(&self, owner: String, repo: String, artifact_id: String) -> Result<GithubArtifact, Error>;
}

#[async_trait]
impl GithubHandlingTrait for GithubHandler {
    fn new(access_token: String) -> Self {
        let client = Client::builder().default_headers(get_default_headers(access_token)).build().unwrap_or_default();
        GithubHandler { client, base_url: "https://api.github.com".to_string() }
    }

    /// parse github time to NaiveDateTime
    fn github_time_parse(&self, timestring: String) -> ParseResult<NaiveDateTime> {
        NaiveDateTime::parse_from_str(&timestring, "%Y-%m-%dT%H:%M:%SZ")
    }

    /// get list of artifacts
    async fn get_list_of_artifacts(&self, owner: String, repo: String) -> Result<Option<GithubArtifacts>, Error> {
        match self.client.get(format!("{}/repos/{}/{}/actions/artifacts", self.base_url, owner, repo)).send().await {
            Err(e) => return Err(e),
            Ok(response) => { 
                match response.json::<TempGithubArtifacts>().await {
                    Ok(object) => {
                        if object.total_count == 0 {
                            return Ok(None);
                        }
                        let mut artifacts = vec!();
                        for i in object.artifacts.iter() {
                            artifacts.push(
                                GithubArtifact{
                                    id: i.clone().id,
                                    node_id: i.clone().node_id,
                                    name: i.clone().name,
                                    size_in_megabytes: i.clone().size_in_megabytes,
                                    url: i.clone().url,
                                    archive_download_url: i.clone().archive_download_url,
                                    expired: i.clone().expired,
                                    created_at: self.github_time_parse(i.clone().created_at).unwrap(),
                                    expires_at: self.github_time_parse(i.clone().expires_at).unwrap(),
                                    updated_at: self.github_time_parse(i.clone().updated_at).unwrap()
                                }
                            );
                        }
                        return Ok(Some(GithubArtifacts {
                            total_count: object.total_count,
                            artifacts
                        }))
                    },
                    Err(e) => return Err(e)
                };
            }
        };
    }

    /// get artifact
    async fn get_artifact(&self, owner: String, repo: String, artifact_id: String) -> Result<GithubArtifact, Error> {
        match self.client.get(format!("{}/repos/{}/{}/actions/artifacts/{}", self.base_url, owner, repo, artifact_id)).send().await {
            Ok(response) => {
                match response.json::<TempGithubArtifact>().await {
                    Ok(i) => {
                        Ok(GithubArtifact{
                            id: i.clone().id,
                            node_id: i.clone().node_id,
                            name: i.clone().name,
                            size_in_megabytes: i.clone().size_in_megabytes,
                            url: i.clone().url,
                            archive_download_url: i.clone().archive_download_url,
                            expired: i.clone().expired,
                            created_at: self.github_time_parse(i.clone().created_at).unwrap(),
                            expires_at: self.github_time_parse(i.clone().expires_at).unwrap(),
                            updated_at: self.github_time_parse(i.clone().updated_at).unwrap()
                        })
                    },
                    Err(err) => return Err(err)
                }
            }
            Err(e) => return Err(e)
        }
    }
}