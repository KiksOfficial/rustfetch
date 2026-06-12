mod get_cpu;
mod get_gpu;
mod get_ram;
mod get_uptime;

use get_cpu::about_cpu;
use get_gpu::about_gpu;
use get_ram::get_ram;
use get_uptime::get_uptime;
fn main() {
    println!("Hello, world!");
    let _ = get_uptime();
    match about_cpu() {
        Ok(n) => println!("{}", n),
        Err(e) => eprintln!("{}", e),
    }
    let _ = about_gpu();
    let _ = get_ram();
}
