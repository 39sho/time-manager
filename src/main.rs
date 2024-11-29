use std::{env, process};
use work::time_manage;

fn main() {
    let args: Vec<String> = env::args().collect();
    execute(args);
}

fn execute(command: Vec<String>) {
    if command.len() != 2 {
        eprintln!("Usage: work <in|out|result>");
        process::exit(1);
    }

    let action = &command[1];

    match action.as_str() {
        "in" => time_manage::work_start(),
        "out" => time_manage::work_end(),
        "result" => time_manage::work_result(),
        _ => {
            eprintln!("Invalid action: {}", action);
            process::exit(1);
        }
    }
}
