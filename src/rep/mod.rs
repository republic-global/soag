use std::{fs, path::PathBuf};
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

    pub fn separate(&self, target: &PathBuf) {
        //TODO: This could be abstracted to a prerequisites() fn
        if let Err(e) = utils::validate_git_repo(&self.directory) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        };

        if let Err(e) = utils::create_rep_directory(&self.directory) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        };

        if let Err(e) = utils::validate_gitignore(&self.directory) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        };

        if let Err(e) = self.add_to_rep(target) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        let separated_repo_path = self.directory.join(".rep/").join(target);

        if let Err(e) = git::init(&&separated_repo_path) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Err(e) = git::add_all(&separated_repo_path) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        if let Err(e) = git::commit(
            &separated_repo_path,
            format!("{} repository created", target.to_str().unwrap()),
        ) {
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

        if let Err(e) = git::add_subtree(&self.directory, target.to_str().unwrap()) {
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
}
