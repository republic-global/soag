use soag::soag::Soag;
use std::path::PathBuf;
use structopt::StructOpt;

mod api;
mod config;
mod git;
mod output;
mod soag;
mod utils;

//TODO: Move to a separated mod?
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
        #[structopt(
            long,
            help = "[Optional] name for creating a new GH repository with.\nGitHub access_token must be set in the config. See `config -h`"
        )]
        github: Option<String>,
    },

    #[structopt(
        about = "Sets up the required configurations which is saved at `~/.soagconfig`.\nSee `soag config -h` for more details",
        name = "config"
    )]
    Configure {
        #[structopt(
            long = "set-github-token",
            help = "Sets the GitHub Access Token in the configuration file (~/.soagconfig)"
        )]
        ght: Option<String>,
        #[structopt(short, help = "Interactive setup")]
        interactive: Option<bool>,
    },
}

//TODO: Add a pretty banner for output
fn main() {
    let opt = Opt::from_args();
    let soag = Soag::new(std::env::current_dir().expect("Failed to get current dir"));

    match opt {
        Opt::Separate { target, github } => {
            let mut flags = Vec::new();
            if let Some(gh) = github {
                flags.push(soag::flags::Flag::GitHub(gh));
            }

            soag.separate(&target, flags);
        }
        Opt::Configure { ght, interactive } => soag.config(ght, interactive),
    }
}
