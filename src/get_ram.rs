use std::fs;

pub fn get_ram() -> std::io::Result<String> {
    let mut total_ram: u64 = 0;
    let mut available_ram: u64 = 0;
    let mut active_ram = 0;

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
                "Active" => active_ram = kb,
                _ => {}
            }
        }
    }

    if total_ram == 0 {
        return Err(std::io::Error::other("MemTotal not found"));
    }

    let used_percent = active_ram as f64 / total_ram as f64 * 100.0;
    let active_ram_gib = active_ram as f64 / 1024.0 / 1024.0;
    let total_ram_gib = total_ram as f64 / 1024.0 / 1024.0;

    Ok(format!(
        "{:.2} GiB / {:.2} GiB ({:.0}%)",
        active_ram_gib, total_ram_gib, used_percent
    ))
}
