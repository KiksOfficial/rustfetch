use std::fs;

pub fn get_ram() -> std::io::Result<()> {
    let mut total_ram: u64 = 0;
    let mut free_ram: u64 = 0;

    let meminfo = fs::read_to_string("/proc/meminfo")?;

    for line in meminfo.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let kb = value
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);

            match key {
                "MemTotal" => total_ram = kb,
                "MemFree" => free_ram = kb,
                _ => {}
            }
        }
    }

    let vastus = (1.0 - free_ram as f64 / total_ram as f64) * 100.0;

    println!("{}%", &vastus.round());

    Ok(())
}
