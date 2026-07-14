use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let info = os_info::get();

    let os = info.os_type();

    let kernel = System::kernel_version();

    let host = System::host_name().unwrap_or_default();

    let cpus = sys.cpus();
    let cpu = cpus[0].brand();
    let cores = cpus.len();

    println!("OS: {}", os);
    match kernel {
        Some(version) => println!("Kernel: {}", version),
        None => println!("Could not get kernel version"),
    }
    println!("Host: {}", host);
    println!("CPU: {}", cpu);
    println!("Cores: {}", cores);
}
