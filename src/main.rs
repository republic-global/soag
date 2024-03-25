use rep::Rep;
use std::path::PathBuf;
use structopt::StructOpt;

mod git;
mod rep;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "SOAG (Son Of A Git)",
    about = "Git repositories management tool"
)]
enum Opt {
    #[structopt(about = "Separate target location into a new repository")]
    Separate {
        #[structopt(help = "Path to the target directory to separate into a repo")]
        target: PathBuf,
        #[structopt(help = "[Optional] url for the new repo")]
        url: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    let rep = Rep::new(std::env::current_dir().expect("Failed to get current dir"));

    match opt {
        Opt::Separate {
            target,
            url: repo_path,
        } => rep.separate(&target, repo_path),
    }
}
