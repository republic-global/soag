use termion::color::{Blue, Fg, Red, Reset};

//TODO: update outputs to use this mod

pub fn error(error: &str) {
    eprintln!("{}{}{}", Fg(Red), error, Fg(Reset));
}

pub fn success(msg: &str) {
    println!("{}{}{}", Fg(Blue), msg, Fg(Reset));
}
