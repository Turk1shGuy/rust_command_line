use std::fs;
use std::path::Path;
use colored::*;

pub fn find(args: Vec<String>) {
    if args.len() != 5 {
        println!("Please enter exactly 5 arguments.");
        crate::help();
        return;
    }

    let dir = &args[2];
    let fdtype = &args[3];
    let name = &args[4];

    println!("Searching in directory: {}", dir.blue());
    println!("Looking for type: {}", fdtype.blue());
    println!("Searching for name: {}", name.blue());

    // Start the recursive search
    if let Err(e) = search_dir(Path::new(dir), fdtype, name) {
        eprintln!("Error during search: {}", e.to_string().red());
    }
}

fn search_dir(dir: &Path, fdtype: &str, name: &str) -> std::io::Result<()> {
    // Read the directory entries
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path: std::path::PathBuf = entry.path();
        let fname = entry.file_name();

        // Check if the name matches
        if let Some(fname_str) = fname.to_str() {
            if fname_str == name {
                match entry.metadata() {
                    Ok(meta) => {
                        if meta.is_dir() && fdtype == "d" {
                            println!("\t{}/", path.display().to_string().green());
                        } else if meta.is_file() && fdtype == "f" {
                            println!("\t{}", path.display().to_string().green());
                        }
                    }
                    Err(e) => eprintln!("Error getting metadata: {}", e),
                }
            }
        } else {
            eprintln!("File name is not valid UTF-8");
        }

        // If it's a directory, search recursively
        if entry.file_type()?.is_dir() {
            search_dir(&path, fdtype, name)?;
        }
    }
    return Ok(());
}
