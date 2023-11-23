use owo_colors::OwoColorize;
use regex::Regex;
use std::{error::Error, io::Write, sync::OnceLock};

use super::{ProcessInfo, ProcessManager};

pub struct ProcessMangerLinux;

impl ProcessManager for ProcessMangerLinux {
    fn get_pids(port: u16) -> Result<Vec<ProcessInfo>, Box<dyn Error>> {
        static INSTANCE: OnceLock<Regex> = OnceLock::new();
        let regex_ports =
            INSTANCE.get_or_init(|| Regex::new(r"(?m)^(?<app>[^\s]*)\s*(?<pid>\d{1,5})").unwrap());

        let output = std::process::Command::new("lsof")
            .arg("-i")
            .arg(format!(":{port}"))
            .output()?;

        let stdout_str = String::from_utf8(output.stdout)?;
        let captures = regex_ports.captures_iter(&stdout_str);

        let pids: Vec<ProcessInfo> = captures
            .map(|cap| {
                cap.name("pid")
                    .map(|pid| {
                        Ok(ProcessInfo {
                            pid: pid.as_str().parse::<u32>().map_err(|_| "Invalid PID")?,
                            name: cap.name("app").ok_or("No app name")?.as_str().to_string(),
                        })
                    })
                    .ok_or("No PID")?
            })
            .collect::<Result<Vec<ProcessInfo>, &'static str>>()?;

        Ok(pids)
    }

    fn kill_pids(processes: &[ProcessInfo]) -> Result<(), Box<dyn Error>> {
        for (idx, process) in processes.iter().enumerate() {
            if idx > 0 {
                println!();
            }
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
}
