use std::path::PathBuf;

use structopt::StructOpt;

///Enum for the accepted commands for soag tool
///For example:
///```soag [COMMAND]```
#[derive(Debug, StructOpt)]
#[structopt(
    name = "SOAG (Son Of A Git)",
    about = "Git repositories management tool"
)]
pub enum Opt {
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
        #[structopt(short, long, help = "Run interactive config setup")]
        interactive: bool,
    },
}
