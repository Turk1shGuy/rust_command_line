pub fn rm(args: Vec<String>) {
    if args.len() < 3 {
        println!("Usage: rm <file_or_dir1> <file_or_dir2> ...");
        return;
    }

    for path in &args[2..] {
        if std::fs::metadata(path).is_ok() {
            match std::fs::remove_file(path) {
                Ok(_) => {}
                Err(_) => match std::fs::remove_dir_all(path) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error removing {}: {}", path, e);
                    }
                },
            }
        } else {
            println!("No such file or directory: {}", path);
        }
    }
}
