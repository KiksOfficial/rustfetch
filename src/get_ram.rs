use std::fs;

pub fn get_ram() -> std::io::Result<String> {
    let mut total_ram: u64 = 0;
    let mut available_ram: u64 = 0;

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
                "MemAvailable" => available_ram = kb,
                _ => {}
            }
        }
    }

    if total_ram == 0 {
        return Err(std::io::Error::other("MemTotal not found"));
    }

    let used_percent = (total_ram - available_ram) as f64 / total_ram as f64 * 100.0;

    Ok(format!("{:.0}%", used_percent))
}
