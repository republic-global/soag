use std::{fs::File, io::BufRead, io::BufReader};

pub fn entry_exists(file: &File, entry: &str) -> bool {
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());

    //It is breaking here
    while let Some(line) = lines.next() {
        if line.trim() == entry.trim() {
            return true;
        }
    }

    false
}
