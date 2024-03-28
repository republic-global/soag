use crate::{
    output::output,
    utils::utils::{self},
};
use std::fs::{self};

const CONFIG_FILE: &str = ".soagconfig";

#[derive(Debug, Clone)]
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

        if self.interactive.is_some_and(|i| i == true) {
            if let Err(e) = self.interactive_setup() {
                output::error(&e.to_string());
            }

            return;
        }

        if let Err(e) = self.save() {
            output::error(&e.to_string());
            return;
        }
    }

    fn save(&self) -> Result<(), std::io::Error> {
        match utils::get_home_dir() {
            Ok(home_dir) => {
                let config_file = home_dir.join(CONFIG_FILE);
                let mut file = utils::file_as_string(&config_file);

                if let Some(ght) = self.ght.clone() {
                    let entry = format!("[github]\n\taccess_token = {}\n", ght);
                    if file.find("[github]").is_some() {
                        let update_ght = entry;
                        file = file.replace("[github]", &update_ght);
                    } else {
                        file.push_str(&entry);
                    }
                }

                utils::save_to_file(&config_file, file.as_str())?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn interactive_setup(&self) -> Result<(), std::io::Error> {
        //prompt each configuration field and save it to disk
        todo!()
    }

    fn validate_config_file(&self) -> Result<(), std::io::Error> {
        match utils::get_home_dir() {
            Ok(home_dir) => {
                let config_file = home_dir.join(CONFIG_FILE);

                if !config_file.exists() {
                    fs::File::create(config_file)?;
                }

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
