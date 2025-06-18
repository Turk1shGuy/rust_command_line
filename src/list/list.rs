use colored::*;
use rust_command_line::help;

pub fn list(args: Vec<String>) {
    if args.len() != 3 {
        help();
        return;
    }

    println!("{}", &args[2]);

    match std::fs::read_dir(&args[2]) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(fentry) => {
                        if fentry.path().is_dir() {
                            println!("\t{}/", fentry.file_name().to_str().unwrap().green());
                        } else {
                            println!("\t{}", fentry.file_name().to_str().unwrap().green());
                        }
                    }
                    Err(e) => println!("Error: {}", e.to_string()),
                }
            }
        }
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
