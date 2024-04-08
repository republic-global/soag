use serde::{Deserialize, Serialize};

use super::{endpoints, helpers};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectRequestData {
    name: String,
    description: String,
    path: String,
    namespace_id: String,
    initialize_with_readme: bool,
}

impl ProjectRequestData {
    ///Creates the required body for a POST request `RepoData`.
    ///All the options are set to default (empty) except for
    ///the name
    pub fn from_name(name: &str) -> Self {
        ProjectRequestData {
            name: name.to_string(),
            description: String::new(),
            path: String::new(),
            namespace_id: String::new(),
            initialize_with_readme: false,
        }
    }
}

pub fn create_new_project(data: ProjectRequestData) -> Result<serde_json::Value, reqwest::Error> {
    let token = helpers::get_config_access_token();

    let res = reqwest::blocking::Client::new()
        .post(endpoints::CREATE_PROJECT)
        .header("Authorization", format!("Bearer {}", token))
        .json(&data)
        .send()?
        .error_for_status()?;

    println!("Response: {:?}", res);

    Ok(res.json()?)
}
