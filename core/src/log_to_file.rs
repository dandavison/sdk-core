use colored::*;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_to_file(msg: &str, prefix: &str, color: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/log")
        .unwrap();

    let msg = format!("{} {}", prefix, msg);
    let msg = match color {
        "red" => msg.red(),
        "green" => msg.green(),
        "blue" => msg.blue(),
        "yellow" => msg.yellow(),
        _ => msg.blue(),
    };

    if let Err(e) = writeln!(file, "{}", msg) {
        eprintln!("Couldn't write to file: {}", e);
    }
}