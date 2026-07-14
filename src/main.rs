use std::env;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let info = os_info::get();

    println!("OS: {}", info.os_type());
    match System::kernel_version() {
        Some(version) => println!("Kernel: {}", version),
        None => println!("Could not get kernel version"),
    }
    println!("Host: {}", System::host_name().unwrap_or_default());
    println!("CPU: {}", sys.cpus()[0].brand());
    println!("Cores: {}", sys.cpus().len());
    match env::var("SHELL") {
        Ok(shell) => println!("Shell: {}", shell),
        Err(_) => println!("SHELL not set"),
    }
}
