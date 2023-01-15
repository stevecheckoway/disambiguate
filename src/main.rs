use colored::*;

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    loop {
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                colorize(&line);
                line.clear();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn colorize(line: &str) {
    for ch in line.chars() {
        if ch.is_uppercase() {
            print!("{}", ch.to_string().yellow());
        } else if ch.is_ascii_digit() {
            print!("{}", ch.to_string().blue());
        } else if ch.is_ascii_punctuation() {
            print!("{}", ch.to_string().red());
        } else {
            print!("{}", ch);
        }
    }
}
