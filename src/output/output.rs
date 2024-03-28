use termion::color::{Fg, Red, Reset};

//TODO: update outputs to use this mod

pub fn error(error: &str) {
    eprintln!("{}{}{}", Fg(Red), error, Fg(Reset));
}
