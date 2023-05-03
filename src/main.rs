mod manager;

use manager::{Manager, ProcessManager};
use owo_colors::OwoColorize;
use std::{error::Error, num::ParseIntError};

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: {}", "killport <port>".green());
        return Ok(());
    }
    let port = args
        .get(1)
        .map(|s| -> Result<u16, ParseIntError> { s.parse::<u16>() })
        .unwrap()?;

    let pids = Manager::get_pids(port)?;
    if pids.is_empty() {
        println!("No processes found on port {}", port.yellow());
        return Ok(());
    }

    Manager::kill_pids(&pids)?;

    Ok(())
}
