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

    pub fn file_stringify() -> Result<String, std::io::Error> {
        match utils::get_home_dir() {
            Ok(home_dir) => {
                let config_file = home_dir.join(CONFIG_FILE);
                Ok(utils::read_file_as_string(&config_file))
            }
            Err(e) => Err(e),
        }
    }

    fn save(&self) -> Result<(), std::io::Error> {
        match utils::get_home_dir() {
            Ok(home_dir) => {
                let config_file = home_dir.join(CONFIG_FILE);
                let mut file = utils::read_file_as_string(&config_file);

                self.update_ght(&mut file);

                utils::save_to_file(&config_file, file.as_str())?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    //TODO: Refactor this plx
    fn update_ght(&self, contents: &mut String) {
        if let Some(ght) = self.ght.clone() {
            if let Some(gh_section) = contents.find("[github]") {
                if let Some(ac_token) = contents[gh_section..].find("access_token") {
                    if let Some(end) = contents[ac_token..].find("\n") {
                        contents.replace_range(
                            ac_token..ac_token + end,
                            &format!("access_token = {}", ght),
                        );
                    }
                } else {
                    let new_line = contents[gh_section..].find("\n").unwrap();
                    contents.replace_range(
                        gh_section..new_line + 1,
                        &format!("[github]\n\taccess_token = {}\n", ght).to_string(),
                    );
                }
            } else {
                contents.push_str(&format!("[github]\n\taccess_token = {}\n", ght));
            }
        }
    }

    fn interactive_setup(&self) -> Result<(), std::io::Error> {
        //prompt each configuration field and save it to disk
        todo!()
    }

    ///Checks if a `.soagconfig` file exists at
    ///user's home directory.
    ///If it doesn't it will try to create one
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
