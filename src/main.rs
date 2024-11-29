use std::{env, process};
use work::time_manage;

fn main() {
    let args: Vec<String> = env::args().collect();
    execute(args);
}

fn execute(command: Vec<String>) {
    if command.len() != 2 {
        eprintln!("Usage: work <init|in|out|result>");
        process::exit(1);
    }

    let action = &command[1];

    match action.as_str() {
        "init" => time_manage::init(),
        "in" => time_manage::start(),
        "out" => time_manage::end(),
        "result" => time_manage::result(),
        _ => {
            eprintln!("Invalid action: {}", action);
            process::exit(1);
        }
    }
}
