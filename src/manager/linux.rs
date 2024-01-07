use owo_colors::OwoColorize;
use regex::Regex;
use std::{error::Error, sync::OnceLock};

use crate::utils::ask_confirmation;

use super::{ProcessInfo, ProcessManager};

pub struct ManagerLinux;

impl ProcessManager for ManagerLinux {
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

        let pids = captures
            .map(|cap| {
                cap.name("pid")
                    .map(|pid| {
                        Ok(ProcessInfo {
                            pid: pid.as_str().parse::<u64>().map_err(|_| "Invalid PID")?,
                            name: Box::<str>::from(cap.name("app").ok_or("No app name")?.as_str()),
                        })
                    })
                    .ok_or("No PID")?
            })
            .collect::<Result<Vec<_>, &'static str>>()?;

        Ok(pids)
    }

    fn kill_pids(processes: &[ProcessInfo]) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();

        for (idx, process) in processes.iter().enumerate() {
            if idx > 0 {
                println!();
            }
            // ask user for confirmation
            let text = format!(
                "Kill process {} with PID {}?",
                process.name.magenta(),
                process.pid.magenta()
            );
            if !ask_confirmation(&text, &mut input)? {
                println!(
                    "Skipping process {} with PID {}",
                    process.name.magenta(),
                    process.pid.magenta()
                );
                continue;
            }

            let _ = std::process::Command::new("kill")
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
