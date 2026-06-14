use std::{fs, io};

pub fn about_gpu() -> io::Result<String> {
    let dir = "/proc/driver/nvidia/gpus";
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries {
            let p = e?.path().join("information");
            if p.exists() {
                let s = fs::read_to_string(&p)?;
                for line in s.lines() {
                    if line.starts_with("Model") {
                        if let Some((_, model)) = line.split_once(':') {
                            return Ok(model.trim().to_string());
                        }
                    }
                }
            }
        }
    }
    println!("No NVIDIA /proc info found");
    Ok("No gpu found".to_string())
}
