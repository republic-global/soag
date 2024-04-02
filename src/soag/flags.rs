use crate::api::{self, github::RepoData};

pub enum Flag {
    GitHub(String),
}

impl Flag {
    pub fn init(&self) {
        match self {
            Flag::GitHub(name) => api::github::create_new_repo(RepoData::from_name(&name)).unwrap(),
        };
    }
}
