use std::fs;
use std::process::{Command};

use serde::Deserialize;
use crate::utils::is_mounted;

#[derive(Deserialize)]
struct Opt {
    #[serde(default)]
    server: String,
    #[serde(default)]
    path: String,
    #[serde(default = "default_protocol")]
    protocol: String,
    #[serde(default)]
    atime: i32,
    #[serde(default)]
    readonly: bool,
}

fn default_protocol() -> String {
    String::from("tcp")
}

impl Opt {
    fn check(&self) -> Result<(), String> {
        if self.protocol != *"tcp" && self.protocol != *"udp" {
            return Err(format!("invalid protocol: {}", self.protocol));
        }

        if self.server.is_empty() {
            return Err("invalid nfs server".to_string());
        }

        if self.path.is_empty() {
            return Err("invalid nfs path".to_string());
        }

        Ok(())
    }

    fn build_nfs_opt(&self) -> String {
        let mut s = format!("{},_netdev,soft,timeo=10,intr", self.protocol);
        if self.atime == 0 {
            s.push_str(",noatime");
        }

        if self.readonly {
            s.push_str(",ro");
        }

        s
    }

    fn build_mount_command(&self, mount_path: &str) -> Command {
        let mut c = Command::new("mount");
        c.arg("-t nfs")
            .arg(format!("-o{}", self.build_nfs_opt()))
            .arg(format!("{}:{}", self.server, self.path))
            .arg(mount_path);
        c
    }
}

pub fn mount(args: &[String]) -> Result<(), String> {
    let mount_path = args.get(2).ok_or_else(|| "can't find mount path".to_string())?;
    let mount_opt_str = args.get(3).ok_or_else(|| "can't find mount options".to_string())?;
    let opt: Opt = serde_json::from_str(mount_opt_str).map_err(|e| e.to_string())?;
    opt.check()?;
    // 检查是否已经挂载
    if is_mounted(mount_path) {
        return Ok(());
    }

    // 创建挂载目录
    fs::create_dir_all(mount_path).map_err(|_e| format!("create dir {} failed", mount_path))?;

    // 挂载
    opt
        .build_mount_command(mount_path)
        .status()
        .map_err(|e| e.to_string())?
        .success()
        .then(|| ())
        .ok_or_else(|| format!("mount {} failed", mount_path))
}
