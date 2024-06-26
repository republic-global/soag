use std::{io::Error, path::PathBuf};

use crate::{git, soag::utils::force_remove};

mod api;
mod endpoints;
mod helpers;

pub fn setup_remote_worktree(dir: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let repo_data = api::RepoData::from_name(name);
    let root_dir = std::env::current_dir().unwrap();

    match api::create_new_repo(repo_data) {
        Ok(res) => {
            let repo_name = res["full_name"].as_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to get GitHub repo URL")
            })?;

            let url = ["git@github.com:", repo_name, ".git"].join("");

            git::add_remote_origin(dir, &url.to_string())?;
            git::move_branch(dir, "master")?;
            git::push(dir, "master")?;
            force_remove(dir)?;
            git::add_all(&root_dir)?;
            git::commit(
                &root_dir,
                format!("{:?} Separated to its own repository", dir),
            )?;
            git::add_subtree(&root_dir, name, Some(url.to_string()), "master")?;

            Ok(())
        }
        Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
    }
}
