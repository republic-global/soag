use crate::{
    config::config::Config,
    output::output::{self},
    soag::utils::{self},
};
use std::{fs, path::PathBuf};
use termion::color::{Fg, Red};

use crate::git;

use super::flags::Flag;

pub struct Soag {
    directory: PathBuf,
}

impl Soag {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    ///Separates the target into a new repository.
    ///it calls `init` function from the Flags enum
    ///for each of the passed flags
    //TODO: cleanup function if there are errors
    pub fn separate(&self, target: &PathBuf, flags: Vec<Flag>) {
        if let Err(e) = self.setup_separation(&target) {
            eprintln!("{}{}{}", Fg(Red), e, Fg(termion::color::Reset));
            return;
        }

        self.init_flags(&flags);

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

        output::success(&format!(
            "SOAG: {} repository separated",
            target.to_str().unwrap()
        ));
    }

    pub fn config(&self, ght: Option<String>, interactive: Option<bool>) {
        Config::new()
            .with_ght(ght)
            .with_interactive_setup(interactive)
            .setup();
    }

    fn init_flags(&self, flags: &Vec<Flag>) {
        for flag in flags {
            flag.init();
        }
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

    ///Sets up the pre-requisites for creating a sub-tree.
    ///This include:
    ///- Having a root repo
    ///- Having a child repo at target's location
    ///- Having a commit in the child repo
    ///
    ///This function tries to do that by calling the relevant git functions
    ///and throws and error if it is unsuccessful to do so.
    fn setup_separation(&self, target: &PathBuf) -> Result<(), std::io::Error> {
        utils::validate_git_repo(&self.directory)?;
        git::init(&self.directory.join(target))?;
        git::add_all(&self.directory.join(target))?;
        git::commit(
            &self.directory.join(target),
            format!("{} repository initialized", target.to_str().unwrap()),
        )?;

        Ok(())
    }

    ///Sets up the remote origin for the target.
    ///This is a wrap function to handle `add origin`
    ///`set-upstream`, deleting the `target` and
    ///catching possible errors in that process
    fn setup_remote_origin(&self, target: &PathBuf, url: &String) -> Result<(), std::io::Error> {
        git::add_remote_origin(&self.directory.join(target), &url)?;
        git::push_set_upstream(&self.directory.join(target), "master")?;
        utils::force_remove(&self.directory.join(target))?;

        Ok(())
    }

    ///This is a wrap function for catching errors
    ///during the process of:
    ///- Creating a `.rep/` directory
    ///- moving the target into it
    ///- adding `.rep/` to the .gitignore (or creating one if it doesn't exist)
    fn setup_local_rep(&self, target: &PathBuf) -> Result<(), std::io::Error> {
        self.add_to_rep(target)?;
        utils::validate_gitignore(&self.directory)?;

        Ok(())
    }
}
