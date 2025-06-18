use rust_command_line::help;

mod echo;
mod find;
mod grep;
mod list;
mod read;
mod clear;
mod mkdir;
mod rm;
mod whoami;
mod pwd;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "echo" => echo::echo::echo(args),
        "read" => read::read::read(args),
        "list" => list::list::list(args),
        "find" => find::find::find(args),
        "grep" => grep::grep::grep(args),
        "clear" => clear::clear::clear(),
        "mkdir" => mkdir::mkdir::mkdir(args),
        "rm" => rm::rm::rm(args),
        "whoami" => whoami::whoami::whoami(),
        "pwd" => pwd::pwd::pwd(),
        _ => help(),
    }

}
