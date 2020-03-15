extern crate systemstat;

use systemstat::{System, Platform};

fn main() {
    let sys = System::new();

    match sys.uptime() {
        Ok(uptime) => println!("\nUptime: {:?}", uptime),
        Err(x) => println!("\nUptime: error: {}", x)
    }

}
