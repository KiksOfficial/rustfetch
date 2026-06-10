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
    let kogu_aeg: i32 = buff_vec[0].parse::<f64>().unwrap() as i32;
    let tunnid = kogu_aeg / 3600;
    let minutes = (kogu_aeg % 3600) / 60;
    let seconds = kogu_aeg % 60;

    println!("{:?}h {:?}min {:?}s", &tunnid, &minutes, &seconds);

    Ok(())
}
