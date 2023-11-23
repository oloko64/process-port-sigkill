mod linux;

use std::error::Error;

#[cfg(target_os = "linux")]
pub use linux::ProcessMangerLinux as Manager;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ProcessInfo {
    pid: u32,
    name: String,
}

pub(super) trait ProcessManager {
    fn get_pids(port: u16) -> Result<Vec<ProcessInfo>, Box<dyn Error>>;
    fn kill_pids(processes: &[ProcessInfo]) -> Result<(), Box<dyn Error>>;
}
