use std::{
    fs::OpenOptions,
    io::{self, BufRead, Error, SeekFrom, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

mod utils;

pub fn repo_exists(dir: &PathBuf) -> bool {
    let git_dir = dir.join(".git");
    git_dir.exists() && git_dir.is_dir()
}

pub fn init(dir: &PathBuf) -> Result<(), Error> {
    let output = Command::new("git")
        .current_dir(dir)
        .arg("init")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initialize git repository",
        ))
    }
}

pub fn gitignore_exists(dir: &PathBuf) -> bool {
    dir.join(".gitignore").exists()
}

pub fn create_gitignore(dir: &PathBuf) -> Result<(), std::io::Error> {
    let output = Command::new("touch")
        .current_dir(dir)
        .arg(".gitignore")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if output.status.success() {
        println!("Git ignore created");
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to create .gitignore",
        ))
    }
}

pub fn add_to_gitignore(dir: &PathBuf, add: &str) -> Result<(), std::io::Error> {
    let gitignore_path = dir.join(".gitignore");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .write(true)
        .open(&gitignore_path)?;

    if !utils::entry_exists(&file, add) {
        writeln!(file, "{}", add)?;
    }

    Ok(())
}
