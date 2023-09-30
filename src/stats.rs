use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

pub fn query_cpu_temp() 
{
    let sys = System::new();

    match sys.memory() {
        Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\nMemory: error: {}", x)
    }

    match sys.boot_time() {
        Ok(boot_time) => println!("\nBoot time: {}", boot_time),
        Err(x) => println!("\nBoot time: error: {}", x)
    }
}
