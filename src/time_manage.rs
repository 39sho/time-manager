use chrono::{DateTime, Duration, Local};
use colored::*;
use csv::{Reader, Writer};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

pub fn init() {
    let now: DateTime<Local> = Local::now();
    let month_dirname = now.format("%Y-%m");
    println!("{month_dirname}");

    fs::create_dir(format!("work/{month_dirname}")).unwrap();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("work/{month_dirname}/start_time_log.csv"))
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(file);

    let _ = wtr
        .write_record(&["start"])
        .expect("Failed to write to CSV");

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("work/{month_dirname}/work_log.csv"))
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(file);

    let _ = wtr
        .write_record(&["start", "end", "duration"])
        .expect("Failed to write to CSV");

    OpenOptions::new()
        .create(true)
        .write(true)
        .open("work/work_state.txt")
        .expect("Failed to open file");
}

pub fn start() {
    let now: DateTime<Local> = Local::now();
    let month_dirname = now.format("%Y-%m");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("work/work_state.txt")
        .expect("Failed to open file");

    file.write(b"working ").expect("Failed to write file");

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(format!("work/{month_dirname}/start_time_log.csv"))
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(file);

    let now: DateTime<Local> = Local::now();
    wtr.write_record(&[now.to_rfc3339()])
        .expect("Failed to write to CSV");
    println!("Work started at {}", now.format("%Y/%m/%d %H:%M"));
}

pub fn end() {
    let now: DateTime<Local> = Local::now();
    let month_dirname = now.format("%Y-%m");

    let _file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("work/work_state.txt")
        .expect("Failed to open file");

    let work_log_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(format!("work/{month_dirname}/work_log.csv"))
        .expect("Failed to open file");

    let start_log_file = OpenOptions::new()
        .read(true)
        .open(format!("work/{month_dirname}/start_time_log.csv"))
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(work_log_file);

    let now: DateTime<Local> = Local::now();
    let mut rdr = Reader::from_reader(start_log_file);

    let record = rdr
        .records()
        .last()
        .unwrap()
        .expect("Failed to read record");
    let last_record: String = record.iter().next().unwrap().to_string();

    let start_time =
        DateTime::parse_from_rfc3339(&last_record).expect("Failed to parse start time");
    let worked_duration: Duration = now.signed_duration_since(start_time);

    wtr.write_record(&[
        last_record.clone(),
        now.to_rfc3339(),
        format!(
            "{}:{:02}",
            worked_duration.num_hours(),
            worked_duration.num_minutes() % 60
        ),
    ])
    .expect("Failed to write to CSV");

    println!(
        "
                        ╭╯         ╭╯
                        ╰╮ Good   ╭╯    {} {}
                        ╭╯ Job! ╭╯
        ▓▓██████████▒ ╭━╯               {}🚬
        ",
        "worked for".bright_cyan().bold(),
        format!(
            "{}:{:02}",
            worked_duration.num_hours(),
            worked_duration.num_minutes() % 60
        )
        .bright_purple()
        .bold(),
        "Wanna go for a smoke?".bright_cyan().bold()
    );
}

pub fn result(month_arg: &String) {
    let work_log_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(format!("work/{month_arg}/work_log.csv"))
        .expect("Failed to open file");

    let mut rdr = Reader::from_reader(work_log_file);

    let results = rdr
        .records()
        .map(|record| record.unwrap().get(2).unwrap().to_owned());

    let mut sum = Duration::new(0, 0).unwrap();

    for result in results {
        println!("{result}");

        let d: Vec<i64> = result
            .split(':')
            .map(|r| r.parse::<i64>().unwrap())
            .collect();

        let duration_min = Duration::hours(d.get(0).unwrap().to_owned());
        let duration_s = Duration::minutes(d.get(1).unwrap().to_owned());

        sum += duration_min;
        sum += duration_s;
    }

    println!("sum: {}:{:02}", sum.num_hours(), sum.num_minutes() % 60);
}

pub fn state() {
    let mut file = OpenOptions::new()
        .read(true)
        .open("work/work_state.txt")
        .expect("Failed to open file");

    let mut state = String::new();
    let _ = file.read_to_string(&mut state);

    print!("{state}");
}
