use std::fs;

pub fn about_cpu() -> std::io::Result<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo")?;

    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            if let Some((_, model)) = line.split_once(':') {
                return Ok(model.trim().to_string());
            }
        }
    }

    Ok("Error".to_string())
}
