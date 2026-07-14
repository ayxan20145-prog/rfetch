use std::io;
use sysinfo::System;

fn main() -> io::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let info = os_info::get();
    let os = info.os_type();

    println!("OS: {}", os);

    Ok(())
}
