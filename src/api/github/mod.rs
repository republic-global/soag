use serde::{Deserialize, Serialize};
mod helpers;

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

pub fn create_new_repo(data: RepoData) -> Result<(), reqwest::Error> {
    let new_repo = RepoData::from_name(&data.name);
    let token = helpers::get_config_access_token();

    let res = reqwest::blocking::Client::new()
        .post("https://api.github.com/user/repo")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", token))
        .body(serde_json::to_string(&new_repo).unwrap())
        .send()?;

    println!("res = {:?}", res);

    Ok(())
}
