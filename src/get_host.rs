use std::fs;

pub fn get_host() -> std::io::Result<String> {
    let os_release = fs::read_to_string("/etc/hostname")?;

    for line in os_release.lines() {
        return Ok(line.to_string());
    }

    Ok("Unknown".to_string())
}
