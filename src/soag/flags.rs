use std::path::PathBuf;

use crate::api::github::{self};

///Accepted flags for the 'separate' command
///For example:
///```soag separate [FLAG]```
///
pub enum Flag {
    GitHub(String),
}

impl Flag {
    pub fn init(&self, target: &PathBuf) {
        match self {
            Flag::GitHub(name) => github::setup_remote_worktree(target, name).unwrap(),
        };
    }
}
