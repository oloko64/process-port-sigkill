mod manager;
mod utils;

use manager::{Manager, ProcessManager};
use owo_colors::OwoColorize;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    if args.len() != 2 {
        println!("Usage: {}", "kp <port>".green());

        return Ok(());
    }
    let port = args
        .nth(1)
        .map(|s| s.parse::<u16>())
        .ok_or("Invalid port")??;

    let pids = Manager::get_pids(port)?;
    if pids.is_empty() {
        println!("No processes found on port {}", port.yellow());
        return Ok(());
    }

    Manager::kill_pids(&pids)?;

    Ok(())
}
