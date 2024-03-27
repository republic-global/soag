use directories::BaseDirs;
use std::fs;
use structopt::StructOpt;
use termion::color::{Fg, LightCyan, Red, Reset};

#[derive(Debug, StructOpt)]
pub struct Config {
    ght: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config { ght: None }
    }

    pub fn set_ght(&mut self, ght: Option<String>) {
        self.ght = ght;
    }

    pub fn setup(&self) {
        if let Err(e) = self.validate_config_file() {
            eprintln!("{}{}{}", Fg(Red), e, Fg(Reset));
            return;
        }
        //check if each of the options already exist
        //ask if they want to overwrite
        //write to ~/.soagconfig
    }

    fn validate_config_file(&self) -> Result<(), std::io::Error> {
        match BaseDirs::new() {
            Some(b_dirs) => {
                let config_path = b_dirs.home_dir().join(".soagconfig");

                if !config_path.exists() {
                    fs::File::create(config_path.clone())?;
                    println!(
                        "{}{:?} created successfully{}",
                        Fg(LightCyan),
                        config_path,
                        Fg(Reset)
                    );
                }

                Ok(())
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}BaseDir failed to initialize{}", Fg(Red), Fg(Reset)),
            )),
        }
    }
}
