use std::fs;

pub fn get_os() -> std::io::Result<String> {
    let os_release = fs::read_to_string("/etc/os-release")?;

    for line in os_release.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            return Ok(value.trim_matches('"').to_string());
        }
    }

    Ok("Unknown".to_string())
}
