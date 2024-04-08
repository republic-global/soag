use crate::{
    output::output,
    utils::utils::{self},
};
use std::fs::{self};

const CONFIG_FILE: &str = ".soagconfig";

#[derive(Debug, Clone)]
pub struct Config {
    ght: Option<String>,
    glt: Option<String>,
    interactive: Option<bool>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ght: None,
            glt: None,
            interactive: None,
        }
    }

    pub fn with_ght(&mut self, ght: Option<String>) -> Self {
        self.ght = ght;
        self.clone()
    }

    pub fn with_glt(&mut self, glt: Option<String>) -> Self {
        self.glt = glt;
        self.clone()
    }

    pub fn with_interactive_setup(&mut self, interactive: bool) -> Self {
        self.interactive = Some(interactive);
        self.clone()
    }

    pub fn setup(&mut self) {
        if let Err(e) = self.validate_config_file() {
            output::error(&e.to_string());
            return;
        }

        //TODO; If no flag provided, handle it
        if self.interactive.is_some_and(|i| i == true) {
            if let Err(e) = self.interactive_setup() {
                output::error(&e.to_string());
                return;
            }
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
                self.update_glt(&mut file);

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
                    if let Some(end) = contents[gh_section + ac_token..].find("\n") {
                        contents.replace_range(
                            gh_section + ac_token..gh_section + ac_token + end,
                            &format!("access_token = {}", ght),
                        );
                    }
                } else {
                    let new_line = contents[gh_section..].find("\n").unwrap();
                    contents.replace_range(
                        gh_section..new_line + 1,
                        &format!("[github]\n\taccess_token = {}\n", ght),
                    );
                }
            } else {
                contents.push_str(&format!("[github]\n\taccess_token = {}\n", ght));
            }
        }
    }

    fn update_glt(&self, contents: &mut String) {
        if let Some(glt) = self.glt.clone() {
            if let Some(gl_section) = contents.find("[gitlab]") {
                if let Some(ac_token) = contents[gl_section..].find("access_token") {
                    if let Some(end) = contents[gl_section + ac_token..].find("\n") {
                        contents.replace_range(
                            gl_section + ac_token..gl_section + ac_token + end,
                            &format!("access_token = {}", glt),
                        );
                    }
                } else {
                    let new_line = contents[gl_section..].find("\n").unwrap();
                    contents.replace_range(
                        gl_section..new_line + 1,
                        &format!("[gitlab]\n\taccess_token = {}\n", glt),
                    );
                }
            } else {
                contents.push_str(&format!("[gitlab]\n\taccess_token = {}\n", glt));
            }
        }
    }

    fn interactive_setup(&mut self) -> Result<(), std::io::Error> {
        let ght = output::prompt("GitHub Access Token: ")?;
        let glt = output::prompt("GitLab Access Token: ")?;

        self.ght = Some(ght);
        self.glt = Some(glt);

        Ok(())
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
