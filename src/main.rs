mod get_cpu;
mod get_gpu;
mod get_ram;
mod get_uptime;

use get_cpu::about_cpu;
use get_gpu::about_gpu;
use get_ram::get_ram;
use get_uptime::get_uptime;

fn main() {
    if let Ok(info) = get_uptime() {
        println!("{}", &info);
    }
    match about_cpu() {
        Ok(n) => println!("{}", n),
        Err(e) => eprintln!("{}", e),
    }
    if let Ok(info) = about_gpu() {
        println!("{}", &info);
    }
    match get_ram() {
        Ok(n) => println!("{}", n),
        Err(e) => eprintln!("{}", e),
    }
}
