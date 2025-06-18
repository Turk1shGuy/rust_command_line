pub fn read(args: Vec<String>) {
    for file in &args[2..] {
        match std::fs::read_to_string(file) {
            Ok(content) => println!("{content}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}

