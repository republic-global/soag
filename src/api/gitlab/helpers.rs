use crate::{config::config::Config, output::output};

pub fn get_config_access_token() -> String {
    let mut acc_token = String::new();

    match Config::file_stringify() {
        Ok(file) => {
            acc_token = parse_access_token(&file);
        }
        Err(e) => {
            output::error(&e.to_string());
        }
    }

    acc_token
}

fn parse_access_token(file: &str) -> String {
    let acc_entry = "access_token = ";

    if let Some(gl_section) = file.find("[gitlab]\n") {
        if let Some(token_section) = file[gl_section..].find(acc_entry) {
            let start = gl_section + token_section + acc_entry.len();
            if let Some(end) = file[start..].find("\n") {
                return file[start..start + end].trim().to_string();
            }
        }
    }

    String::new()
}
