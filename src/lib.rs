use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use regex::Regex;
use std::{error::Error, io::Write, num::ParseIntError};

pub fn get_pids(port: u16) -> Result<Vec<MyProcess>, Box<dyn Error>> {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?m)^(?<app>[^\s]*)\s*(?<pid>\d{1,5})").unwrap());

    let output = std::process::Command::new("lsof")
        .arg("-i")
        .arg(format!(":{port}"))
        .output()?;

    let pids: Vec<MyProcess> = RE
        .captures_iter(&String::from_utf8(output.stdout)?)
        .map(|cap| -> Result<MyProcess, ParseIntError> {
            cap.name("pid")
                .map(|pid| -> Result<MyProcess, ParseIntError> {
                    Ok(MyProcess {
                        pid: pid.as_str().parse()?,
                        name: cap.name("app").unwrap().as_str().to_string(),
                    })
                })
                .unwrap()
        })
        .collect::<Result<Vec<MyProcess>, ParseIntError>>()?;

    Ok(pids)
}

pub fn kill_pids(processes: &[MyProcess]) -> Result<(), Box<dyn Error>> {
    for process in processes {
        // ask user for confirmation
        print!(
            "Kill process {} with PID {}? [y/N]: ",
            process.name.magenta(),
            process.pid.magenta()
        );
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            continue;
        }

        std::process::Command::new("kill")
            .arg("-9")
            .arg(process.pid.to_string())
            .output()?;

        println!(
            "Killed process {} with PID {}",
            process.name.magenta(),
            process.pid.magenta()
        );
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct MyProcess {
    pid: u32,
    name: String,
}
