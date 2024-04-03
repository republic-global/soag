use crate::{
    config::config::Config,
    output::output::{self},
    soag::utils::{self},
};
use std::path::PathBuf;
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

        self.init_flags(&flags, target);

        output::success(&format!(
            "SOAG: {} repository separated",
            target.to_str().unwrap()
        ));
    }

    pub fn config(&self, ght: Option<String>, interactive: bool) {
        Config::new()
            .with_ght(ght)
            .with_interactive_setup(interactive)
            .setup();
    }

    fn init_flags(&self, flags: &Vec<Flag>, target: &PathBuf) {
        for flag in flags {
            flag.init(target);
        }
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
}
