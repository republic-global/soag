use std::io::{self, Write};

//TODO: update outputs to use this mod

pub fn error(error: &str) {
    eprintln!("{}", error);
}

pub fn success(msg: &str) {
    println!("{}", msg);
}

pub fn prompt(msg: &str) -> Result<String, std::io::Error> {
    let mut res = String::new();
    print!("{}", msg);
    io::stdout().flush()?;

    if let Err(e) = std::io::stdin().read_line(&mut res) {
        return Err(e);
    }

    Ok(res)
}
