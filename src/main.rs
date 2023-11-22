use chrono::prelude::*;

fn main () {
    // Format to YYYY-MM-DD HH:MM:SS:MS
    loop {
        print!("\r{}", Local::now().format("%Y-%m-%d %H:%M:%S:%3f"));
        // sleep for 1 nano second
        std::thread::sleep(std::time::Duration::from_nanos(1));
    }
}