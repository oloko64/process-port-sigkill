mod linux;

use std::error::Error;

#[cfg(target_os = "linux")]
pub use linux::ManagerLinux as Manager;

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessInfo {
    pid: u64,
    name: Box<str>,
}

pub trait ProcessManager {
    fn get_pids(port: u16) -> Result<impl IntoIterator<Item = ProcessInfo>, Box<dyn Error>>;
    fn kill_pids(processes: &[ProcessInfo]) -> Result<(), Box<dyn Error>>;
}
