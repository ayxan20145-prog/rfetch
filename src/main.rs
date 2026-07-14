use std::{env, fs};
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let info = os_info::get();

    let os_release = fs::read_to_string("/etc/os-release").unwrap();

    let distro = os_release
        .lines()
        .find(|line| line.starts_with("ID="))
        .unwrap_or("")
        .trim_start_matches("ID=");

    let ascii = match distro {
        "cachyos" => include_str!("ascii/cachyos.txt"),
        _ => include_str!("ascii/default.txt"),
    };

    println!("{}", ascii);

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
