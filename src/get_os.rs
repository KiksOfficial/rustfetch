use std::fs;
use std::path::Path;

pub fn get_os() -> std::io::Result<String> {
    let os_release = fs::read_to_string("/etc/os-release")?;

    for line in os_release.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            return Ok(value.trim_matches('"').to_string());
        }
    }

    Ok("Unknown".to_string())
}

pub fn get_desktop() -> Option<String> {
    std::env::var("XDG_CURRENT_DESKTOP").ok()
}
pub fn get_shell() -> Option<String> {
    let shell_path = std::env::var("SHELL").ok()?;

    Path::new(&shell_path)
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
}
