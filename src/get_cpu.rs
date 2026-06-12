use std::fs;

pub fn about_cpu() -> std::io::Result<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo")?;

    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            return Ok(line.to_string());
        }
    }

    Ok("Error".to_string())
}
