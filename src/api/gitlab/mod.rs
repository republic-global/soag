use crate::{git, soag::utils::force_remove};
use std::path::PathBuf;

mod api;
mod endpoints;
mod helpers;

pub fn setup_remote_worktree(dir: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let repo_data = api::ProjectRequestData::from_name(name);
    let root_dir = std::env::current_dir().unwrap();

    match api::create_new_project(repo_data) {
        Ok(res) => {
            let repo_url = res["ssh_url_to_repo"].as_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to get GitLab repo URL")
            })?;

            git::add_remote_origin(dir, &repo_url.to_string())?;
            git::move_branch(dir, "master")?;
            force_remove(dir)?;
            git::add_all(&root_dir)?;
            git::commit(
                &root_dir,
                format!("{:?} Separated to its own repository", dir),
            )?;
            git::add_subtree(&root_dir, name, Some(repo_url.to_string()))?;

            Ok(())
        }
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}
