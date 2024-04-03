use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

use super::endpoints;
use super::helpers;

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoData {
    name: String,
    description: String,
    homepage: String,
    is_template: bool,
}

impl RepoData {
    ///Creates the required body for a POST request `RepoData`.
    ///All the options are set to default (empty) except for
    ///the name
    pub fn from_name(name: &str) -> Self {
        RepoData {
            name: name.to_string(),
            description: String::new(),
            homepage: String::new(),
            is_template: false,
        }
    }
}

pub fn create_new_repo(data: RepoData) -> Result<serde_json::Value, reqwest::Error> {
    let token = helpers::get_config_access_token();

    let res = reqwest::blocking::Client::new()
        .post(endpoints::CREATE_REPO)
        .header(USER_AGENT, "S.O.A.G.")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .json(&data)
        .send()?
        .error_for_status()?;

    Ok(res.json()?)
}
