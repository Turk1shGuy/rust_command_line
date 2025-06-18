use colored::Colorize;

pub fn grep(args: Vec<String>) {
    let filter = &args[2];
    
    println!("Found:");
    for i in &args[3..] {
        if i.contains(filter) {
            println!("\t{}", i.green());
        }
    }
}
