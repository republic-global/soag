use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::git;

///Checks if a `.git` repo exists at the provided location
///and tries to create one if it doesn't
pub fn validate_git_repo(dir: &PathBuf) -> Result<(), std::io::Error> {
    if !git::repo_exists(dir) {
        git::init(dir)?
    }

    Ok(())
}

pub fn validate_gitignore(dir: &PathBuf) -> Result<(), std::io::Error> {
    if !git::gitignore_exists(dir) {
        git::create_gitignore(dir)?;
    }

    git::add_to_gitignore(dir, ".rep")
}

///Remove passed argument
///runs `rm -rf {path}`
pub fn force_remove(path: &PathBuf) -> Result<(), std::io::Error> {
    let output = Command::new("rm")
        .args(["-rf", path.to_str().unwrap()])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to remove {}", path.to_str().unwrap()),
        ));
    }

    Ok(())
}
