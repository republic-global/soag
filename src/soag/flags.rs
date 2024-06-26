use std::path::PathBuf;

use crate::api::{github, gitlab};

///Accepted flags for the 'separate' command
///For example:
///```soag separate [FLAG]```
///
pub enum Flag {
    GitHub(String),
    GitLab(String),
}

impl Flag {
    pub fn init(&self, target: &PathBuf) {
        match self {
            Flag::GitHub(name) => github::setup_remote_worktree(target, name).unwrap(),
            Flag::GitLab(name) => gitlab::setup_remote_worktree(target, name).unwrap(),
        };
    }
}
