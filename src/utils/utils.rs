use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Write},
    path::PathBuf,
};

use directories::UserDirs;
use termion::color::{Fg, Red, Reset};

pub fn get_home_dir() -> Result<PathBuf, Error> {
    match UserDirs::new() {
        Some(user_dirs) => Ok(user_dirs.home_dir().to_path_buf()),
        None => Err(Error::new(
            ErrorKind::Other,
            format!("{}BaseDirs failed to initialize{}", Fg(Red), Fg(Reset)),
        )),
    }
}

///Returns the requested file contents as a `String`
///or an empty `String` if it is unable to read the file
pub fn read_file_as_string(path: &PathBuf) -> String {
    let mut result = String::new();

    if let Ok(existing_file) = File::open(path) {
        let reader = BufReader::new(existing_file);
        for line in reader.lines() {
            result.push_str(&line.unwrap());
            result.push('\n');
        }
    }

    result
}

pub fn save_to_file(file_path: &PathBuf, contents: &str) -> Result<(), Error> {
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
