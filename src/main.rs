mod get_cpu;
mod get_gpu;
mod get_host;
mod get_os;
mod get_ram;
mod get_uptime;

use get_cpu::about_cpu;
use get_gpu::about_gpu;
use get_host::get_host;
use get_os::get_os;
use get_ram::get_ram;
use get_uptime::get_uptime;

fn main() {
    if let Ok(info) = get_uptime() {
        println!(r"   / \__                    Uptime: {}", &info);
    }
    match about_cpu() {
        Ok(n) => println!(r"  (    @\___                CPU: {}", n),
        Err(e) => eprintln!("{}", e),
    }
    if let Ok(info) = about_gpu() {
        println!(r"  /         O               GPU: {}", &info);
    }
    match get_ram() {
        Ok(n) => println!(r" /   (_____/                Memory: {}", n),
        Err(e) => eprintln!("{}", e),
    }
    if let Ok(info) = get_os() {
        println!(r" /_____/   U                OS: {}", &info);
    }

    if let Ok(info) = get_host() {
        println!(r"                            Host: {}", &info);
    }
}
