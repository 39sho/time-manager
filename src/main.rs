use std::env;
use work::time_manage;

fn main() {
    let args: Vec<String> = env::args().collect();
    time_manage::time_manage(args);
}
