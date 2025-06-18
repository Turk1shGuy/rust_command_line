use colored::Colorize;

pub fn whoami() {
    match std::env::var("USER") {
        Ok(username) => println!("{}", username.green()),
        Err(e) => println!("Username not found: {}", e.to_string()),
    }
}