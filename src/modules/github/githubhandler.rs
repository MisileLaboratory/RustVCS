use crate::modules::base::requesthandler::get_default_headers;
use crate::modules::github::structs::githubstructs::*;

use chrono::format::ParseResult;
use chrono::{NaiveDateTime, Datelike, Timelike};

use reqwest::{Client, Error, Response};

use async_trait::async_trait;

use serde::Deserialize;

pub struct GithubHandler {
    client: Client,
    base_url: String
}

#[derive(Deserialize, Clone)]
struct TempGithubArtifact {
    id: u128,
    node_id: String,
    name: String,
    size_in_megabytes: u128,
    url: String,
    archive_download_url: String,
    expired: bool,
    created_at: String,
    expires_at: String,
    updated_at: String
}

#[derive(Deserialize)]
struct TempGithubArtifacts {
    total_count: u128,
    artifacts: Vec<TempGithubArtifact>
}

#[async_trait]
pub trait GithubHandlingTrait {
    fn new(access_token: String) -> Self;
    fn github_time_parse(&self, timestring: String) -> ParseResult<NaiveDateTime>;
    fn time_to_string(&self, time: NaiveDateTime) -> String;
    async fn get_list_of_artifacts(&self, owner: String, repo: String) -> Result<Option<GithubArtifacts>, Error>;
    async fn get_artifact(&self, owner: String, repo: String, artifact_id: u128) -> Result<GithubArtifact, Error>;
    async fn delete_artifact(&self, owner: String, repo: String, artifact_id: u128) -> Result<(), Error>;
    async fn get_artifact_data(&self, owner: String, repo: String, artifact_id: u128, artifact_format: Option<String>) -> Result<Response, Error>;
    async fn get_artifact_list_from_one(&self, owner: String, repo: String, run_id: u128) -> Result<Option<GithubArtifacts>, Error>;
    async fn get_actions_cache_usage(&self, name: String, enterprise: bool) -> Result<GithubCacheUsage, Error>;
    async fn get_actions_project_cache_usage(&self, owner: String, repo: String) -> Result<GithubProjectCacheUsage, Error>;
}

#[async_trait]
impl GithubHandlingTrait for GithubHandler {
    /// Make new object with access token
    fn new(access_token: String) -> Self {
        let client = Client::builder().default_headers(get_default_headers(access_token)).build().unwrap_or_default();
        GithubHandler { client, base_url: "https://api.github.com".to_string() }
    }

    /// parse github time to NaiveDateTime
    fn github_time_parse(&self, timestring: String) -> ParseResult<NaiveDateTime> {
        NaiveDateTime::parse_from_str(&timestring, "%Y-%m-%dT%H:%M:%SZ")
    }

    fn time_to_string(&self, time: NaiveDateTime) -> String {
        format!("{}-{}-{}T{}:{}:{}Z", time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second())
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
    async fn get_artifact(&self, owner: String, repo: String, artifact_id: u128) -> Result<GithubArtifact, Error> {
        match self.client.get(format!("{}/repos/{}/{}/actions/artifacts/{}", self.base_url, owner, repo, artifact_id)).send().await {
            Ok(response) => {
                match response.json::<TempGithubArtifact>().await {
                    Ok(i) => {
                        Ok(GithubArtifact{
                            id: i.id,
                            node_id: i.clone().node_id,
                            name: i.clone().name,
                            size_in_megabytes: i.size_in_megabytes,
                            url: i.clone().url,
                            archive_download_url: i.clone().archive_download_url,
                            expired: i.expired,
                            created_at: self.github_time_parse(i.clone().created_at).unwrap(),
                            expires_at: self.github_time_parse(i.clone().expires_at).unwrap(),
                            updated_at: self.github_time_parse(i.updated_at).unwrap()
                        })
                    },
                    Err(err) => return Err(err)
                }
            }
            Err(e) => return Err(e)
        }
    }

    /// delete artifact
    async fn delete_artifact(&self, owner: String, repo: String, artifact_id: u128) -> Result<(), Error> {
        match self.client.get(format!("{}/repos/{}/{}/actions/artifacts/{}", self.base_url, owner, repo, artifact_id)).send().await {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err)
        }
    }
    

    /// get artifact data
    async fn get_artifact_data(&self, owner: String, repo: String, artifact_id: u128, artifact_format: Option<String>) -> Result<Response, Error> {
        match self.client.get(format!("{}/repos/{}/{}/actions/artifacts/{}/{}", self.base_url, owner, repo, artifact_id, artifact_format.unwrap_or_else(|| "zip".to_string()))).send().await {
            Ok(data) => return Ok(data),
            Err(err) => return Err(err)
        }
    }

    /// get list of artifacts from one workflow run
    async fn get_artifact_list_from_one(&self, owner: String, repo: String, run_id: u128) -> Result<Option<GithubArtifacts>, Error> {
        match self.client.get(format!("{}/repos/{}/{}/actions/runs/{}/artifacts", self.base_url, owner, repo, run_id)).send().await {
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
            },
            Err(err) => return Err(err)
        }
    }

    /// Gets the total GitHub Actions cache usage.
    /// If enterprise is true, you will get enterprise cache usage else organization
    async fn get_actions_cache_usage(&self, name: String, enterprise: bool) -> Result<GithubCacheUsage, Error> {
        let get_url = if enterprise {
            format!("/enterprises/{}/actions/cache/usage", name)
        } else {
            format!("/orgs/{}/actions/cache/usage", name)
        };
        match self.client.get(format!("{}{}", self.base_url, get_url)).send().await {
            Ok(response) => {
                match response.json::<GithubCacheUsage>().await {
                    Ok(object) => return Ok(object),
                    Err(e) => return Err(e)
                }
            },
            Err(err) => return Err(err)
        }
    }

    // Gets gitHub actions cache usage for a repository.
    async fn get_actions_project_cache_usage(&self, owner: String, repo: String) -> Result<GithubProjectCacheUsage, Error> {
        match self.client.get(format!("{}/repos/{}/{}/actions/cache/usage", self.base_url, owner, repo)).send().await {
            Ok(response) => {
                match response.json::<GithubProjectCacheUsage>().await {
                    Ok(object) => return Ok(object),
                    Err(e) => return Err(e)
                }
            },
            Err(err) => return Err(err)
        }
    }
}