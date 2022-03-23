use std::io::Error;

use crate::modules::base::requesthandler;
use crate::modules::github::structs::githubstructs::*;

use chrono::naive::NaiveDateTime;
use chrono::format::ParseResult;

pub struct GithubHandler {

}

trait GithubHandlingTrait {
    fn github_time_parse(&self) -> Result<ParseResult<NaiveDateTime>, Error>;
    fn get_list_of_artifacts(&self, owner: &str, repo: &str) -> Result<Vec<GithubArtifact>, Error>;
}