use std::fs;

pub fn about_cpu() -> std::io::Result<()> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo")?;

    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            println!("{}", line);
            break;
        }
    }

    Ok(())
}
