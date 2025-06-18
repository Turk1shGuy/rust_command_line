
pub fn echo(args: Vec<String>) {
    for arg in &args[2..] {
        println!("{arg}");
    }
}
