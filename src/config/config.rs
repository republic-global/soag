use crate::{
    output::output,
    utils::utils::{self},
};
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    ght: Option<String>,
    interactive: Option<bool>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ght: None,
            interactive: None,
        }
    }

    pub fn with_ght(&mut self, ght: Option<String>) -> Self {
        self.ght = ght;
        self.clone()
    }

    pub fn with_interactive_setup(&mut self, interactive: Option<bool>) -> Self {
        self.interactive = interactive;
        self.clone()
    }

    pub fn setup(&self) {
        if let Err(e) = self.validate_config_file() {
            output::error(&e.to_string());
            return;
        }

        if let Some(interactive) = self.interactive {
            if interactive {
                if let Err(e) = self.interactive_setup() {
                    output::error(&e.to_string());
                }

                return;
            }
        }

        if let Err(e) = self.save() {
            output::error(&e.to_string());
            return;
        }
    }

    fn save(&self) -> Result<(), std::io::Error> {
        //save current configuratin to disk
        todo!()
    }

    fn interactive_setup(&self) -> Result<(), std::io::Error> {
        //prompt each configuration field and save it to disk
        todo!()
    }

    fn validate_config_file(&self) -> Result<(), std::io::Error> {
        match utils::get_home_dir() {
            Ok(home_dir) => {
                let config_file = home_dir.join(".soagconfig");

                if !config_file.exists() {
                    fs::File::create(config_file)?;
                }

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
