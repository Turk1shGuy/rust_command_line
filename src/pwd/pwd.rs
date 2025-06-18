use colored::*;

pub fn pwd() {
    match std::env::var("PWD") {
        Ok(wd) => println!("{}", wd.green()),
        Err(e) => println!("Working Directories: {}", e.to_string()),
    }
}
