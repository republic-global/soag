use arguments::Opt;
use soag::soag::Soag;
use structopt::StructOpt;

mod api;
mod arguments;
mod config;
mod git;
mod output;
mod soag;
mod utils;

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
