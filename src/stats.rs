use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

pub fn query_cpu_temp() 
{
    let sys = System::new();

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }
}
