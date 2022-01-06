use std::process::Command;

// 是否已经挂载
pub fn is_mounted(mount_path: &str) -> bool {
    match Command::new("findmnt")
        .arg(mount_path)
        .arg("--output")
        .arg("TARGET")
        .arg("--noheadings")
        .status()
    {
        Ok(i) => i.success(),
        _ => false
    }
}
