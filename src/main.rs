use std::{env, process};
use work::time_manage;

fn main() {
    let args: Vec<String> = env::args().collect();
    execute(args);
}

fn execute(command: Vec<String>) {
    if command.len() == 1 {
        eprintln!("Usage: work <init|in|out|result>");
        process::exit(1);
    }

    let action = &command[1];

    match action.as_str() {
        "init" => time_manage::init(),
        "in" => time_manage::start(),
        "out" => time_manage::end(),
        "result" => {
            match &command.get(2) {
                Some(arg) => {
                    time_manage::result(arg);
                }
                None => {
                    println!("Usage: work result YYYY-MM");
                    process::exit(1)
                }
            };
        }
        "state" => time_manage::state(),
        _ => {
            eprintln!("Invalid action: {}", action);
            process::exit(1);
        }
    }
}
