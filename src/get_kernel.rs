use std::fs;

pub fn get_kernel() -> std::io::Result<String> {
    let cpuinfo = fs::read_to_string("/proc/version")?;

    let rida: Vec<&str> = cpuinfo.split_whitespace().collect();

    Ok(rida[2].to_string())
}
