use std::process::Command;

pub fn umount(args: &[String]) -> Result<(), String> {
    let path = args.get(1).ok_or_else(|| "can't find umount path".to_string())?;
    _unmount(path)
}

fn _unmount(path: &str) -> Result<(), String> {
    let s = Command::new("umount")
        .arg(path)
        .status();
    if let Ok(i) = s {
        if i.success() {
            return Ok(());
        }
    }

    Err(format!("Failed to unmount volume at {}", path))
}
