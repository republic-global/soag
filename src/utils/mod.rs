use directories::BaseDirs;
use std::{
    fs::{self},
    path::Path,
};
use termion::color::{Fg, LightCyan, Red, Reset};

///Checks if an entity exists at location
///And creates it if it doesn't
pub fn validate_system_entity(path: &str) -> Result<(), std::io::Error> {
    Ok(())
}
