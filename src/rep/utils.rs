use std::{fs, path::PathBuf};
use termion::color::{Fg, Magenta};

use crate::git;

pub fn create_rep_directory(dir: &PathBuf) -> Result<(), std::io::Error> {
    let rep_dir = dir.join(".rep");

    if !rep_dir.exists() {
        fs::create_dir(&rep_dir)?;
    }

    println!(
        "{}rep dir validated{}",
        Fg(Magenta),
        Fg(termion::color::Reset)
    );

    Ok(())
}

pub fn validate_git_repo(dir: &PathBuf) -> Result<(), std::io::Error> {
    if !git::repo_exists(dir) {
        git::init(dir)?
    }

    println!("Git repo validated");

    Ok(())
}

pub fn validate_gitignore(dir: &PathBuf) -> Result<(), std::io::Error> {
    if !git::gitignore_exists(dir) {
        git::create_gitignore(dir)?;
    }

    git::add_to_gitignore(dir, ".rep")
}
