use std::io::{self, Write};

use termion::color::{Blue, Fg, Red, Reset};

//TODO: update outputs to use this mod

pub fn error(error: &str) {
    eprintln!("{}{}{}", Fg(Red), error, Fg(Reset));
}

pub fn success(msg: &str) {
    println!("{}{}{}", Fg(Blue), msg, Fg(Reset));
}

pub fn prompt(msg: &str) -> Result<String, std::io::Error> {
    let mut res = String::new();
    print!("{}{}{}", Fg(Blue), msg, Fg(Reset));
    io::stdout().flush()?;

    if let Err(e) = std::io::stdin().read_line(&mut res) {
        return Err(e);
    }

    Ok(res)
}
