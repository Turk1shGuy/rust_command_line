pub fn mkdir(args: Vec<String>) {
    if args.len() < 3 {
        println!("Please enter directories names");
    }

    for dir_name in &args[2..] {
        match std::fs::create_dir(dir_name) {
            Ok(_) => {}
            Err(e) => println!("Error: {}", e.to_string()),
        }
    }
}
