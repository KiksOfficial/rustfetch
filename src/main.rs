mod get_cpu;
mod get_gpu;
mod get_uptime;

use get_cpu::about_cpu;
use get_gpu::about_gpu;
use get_uptime::get_uptime;
fn main() {
    println!("Hello, world!");
    let _ = get_uptime();
    let _ = about_cpu();
    let _ = about_gpu();
}
