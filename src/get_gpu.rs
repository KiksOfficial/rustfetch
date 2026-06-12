use std::{fs, io, path::Path};

pub fn about_gpu() -> io::Result<()> {
    let dir = "/proc/driver/nvidia/gpus";
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries {
            let p = e?.path().join("information");
            if p.exists() {
                let s = fs::read_to_string(&p)?;
                for line in s.lines() {
                    if line.starts_with("Model") {
                        println!("{}", line);
                        return Ok(());
                    }
                }
            }
        }
    }
    println!("No NVIDIA /proc info found");
    Ok(())
}
