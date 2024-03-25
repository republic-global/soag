use std::{cell::Ref, fs, path::PathBuf};
use termion::color::{Fg, Magenta, Red};

mod utils;
use crate::git;

pub struct Rep {
    directory: PathBuf,
}

impl Rep {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    ///Separates the target into a new repository
    ///If the `repo` arg is provided, it pushes the recently
    ///created repo to that origin
    pub fn separate(&self, target: &PathBuf, url: Option<String>) {
        if let Err(e) = utils::validate_git_repo(&self.directory) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Err(e) = git::init(&self.directory.join(target)) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Err(e) = git::add_all(&self.directory.join(target)) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Err(e) = git::commit(
            &self.directory.join(target),
            format!("{} repository initialized", target.to_str().unwrap()),
        ) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Some(remote) = url.clone() {
            if let Err(e) = git::add_remote_origin(&self.directory.join(target), &remote) {
                eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
                //TODO: Cleanup function when it fails
                return;
            }

            if let Err(e) = git::push_set_upstream(&self.directory.join(target), "master") {
                eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
                //TODO: Cleanup function when it fails
                return;
            }
        }

        if let Err(e) = utils::force_remove(&self.directory.join(target)) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Err(e) = git::add_all(&self.directory) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Err(e) = git::commit(
            &self.directory,
            format!(
                "{} separated to its own repository",
                target.to_str().unwrap()
            ),
        ) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        //TODO: Create subtree merge squash
        if let Err(e) = git::add_subtree(&self.directory, target.to_str().unwrap(), url) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        println!(
            "{}SOAG: {} repository separated{}",
            Fg(Magenta),
            target.to_str().unwrap(),
            Fg(termion::color::Reset)
        );
    }

    fn add_to_rep(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let rep_dir = self.directory.join(".rep");

        if !rep_dir.exists() {
            fs::create_dir(&rep_dir)?;
        }

        let new_path = rep_dir.join(path.file_name().unwrap());

        fs::rename(path, new_path)?;

        Ok(())
    }

    fn prerequisites_setup(&self) -> Result<(), std::io::Error> {
        utils::validate_git_repo(&self.directory)?;
        // utils::create_rep_directory(&self.directory)?;
        // utils::validate_gitignore(&self.directory)?;

        Ok(())
    }

    fn add_new_repo_setup(&self, repo: &PathBuf) -> Result<(), std::io::Error> {
        let separated_repo_path = self.directory.join(".rep/").join(repo);

        self.add_to_rep(repo)?;
        git::init(&separated_repo_path)?;
        git::add_all(&separated_repo_path)?;
        git::commit(
            &separated_repo_path,
            format!("{} repository created", repo.to_str().unwrap()),
        )?;
        git::add_all(&self.directory)?;
        git::commit(
            &self.directory,
            format!("{} separated to its own repository", repo.to_str().unwrap()),
        )?;
        // git::add_subtree(&self.directory, repo.to_str().unwrap())?;

        Ok(())
    }
}
