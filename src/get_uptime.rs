use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn get_uptime() -> std::io::Result<()> {
    let f = File::open("/proc/uptime")?;
    let reader = BufReader::new(f);
    let mut buff_vec: Vec<String> = Vec::new();

    for rida in reader.lines() {
        match rida {
            Ok(n) => {
                buff_vec = Vec::from(n.split_whitespace().map(String::from).collect::<Vec<_>>())
            }
            Err(e) => println!("{}", e),
        }
    }
    let kogu_aeg: f32 = buff_vec[0].parse().unwrap();
    let tunnid = kogu_aeg / 3600.0;

    println!("{:?}", &tunnid);

    Ok(())
}
