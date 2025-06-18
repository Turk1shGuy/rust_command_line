pub fn clear() {
    match std::process::Command::new("clear").status() {
        Ok(status) if status.success() => {

        }
        Ok(status) => {
            eprintln!("Command exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}
