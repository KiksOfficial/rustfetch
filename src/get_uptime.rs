use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_uptime() -> std::io::Result<String> {
    let f = File::open("/proc/uptime")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let uptime: f64 = line.split_whitespace().next().unwrap().parse().unwrap();

    let secs = uptime as u64;

    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    Ok(format!("{}h {}min {}s", hours, minutes, seconds))
}
