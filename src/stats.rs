use std::thread;
use std::time::Duration;

pub fn query_cpu_temp() 
{
    let sys = systemstat::System::new();

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }
}
