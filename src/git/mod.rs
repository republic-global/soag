use std::{
    io::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn repo_exists(dir: &PathBuf) -> bool {
    let git_dir = dir.join(".git");
    git_dir.exists() && git_dir.is_dir()
}

///Runs `git init` at the passed location
///pipes the output and retunrs
///an error if it fails
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

///Creates a subtree from the provided location.
///Runs `git subtree add --prefix {name} [{url}]`
///If a URL is not provided, we use the `.rep/`
///directory
//TODO: Here we need to get rid of the .rep implementation and
//instead use a 'default' method which we can take from the
//configuration file
pub fn add_subtree(dir: &PathBuf, name: &str, url: Option<String>) -> Result<(), std::io::Error> {
    let mut uri = dir.join(".rep/").join(name).to_string_lossy().into_owned();

    if let Some(u) = url {
        uri = u;
    }

    println!("This is url: {}", uri);

    let output = Command::new("git")
        .args([
            "subtree",
            "add",
            "--prefix",
            name,
            uri.as_str(),
            "master",
            "--squash",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to create subtree",
        ));
    }

    Ok(())
}

///Runs `git add .` at the provided location
///pipes the output and returns an Error
///if something fails
pub fn add_all(dir: &PathBuf) -> Result<(), std::io::Error> {
    let add_output = Command::new("git")
        .current_dir(dir)
        .args(["add", "."])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !add_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to add all changes",
        ));
    }

    Ok(())
}

pub fn commit(dir: &PathBuf, message: String) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["commit", "-m", message.as_str()])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to commit changes for {}", dir.to_str().unwrap()),
        ));
    }

    Ok(())
}

///Runs `git remote add origin {url}`
///pipes the output and returns an error
///if it fails
pub fn add_remote_origin(dir: &PathBuf, url: &String) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["remote", "add", "origin", url])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to add remote origin {}", url),
        ));
    }

    Ok(())
}

pub fn move_branch(dir: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["branch", "-M", name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Unable to move branch {}", name),
        ));
    }

    Ok(())
}

pub fn push(dir: &PathBuf, branch: &str) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["push", "-u", "origin", branch])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Unable to push {}", branch),
        ));
    }

    Ok(())
}
