use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut _str = String::new();
    io::stdin().read_line(&mut _str).unwrap();
    _str
}
