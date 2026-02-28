use std::collections::BTreeMap;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

pub fn run_with_env(command: &[String], envs: BTreeMap<String, String>) -> Result<i32> {
    if command.is_empty() {
        bail!("缺少要执行的命令。示例: envinject run myproj -- npm run dev");
    }

    let mut cmd = Command::new(&command[0]);
    cmd.args(&command[1..]);
    cmd.envs(envs);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let status = cmd
        .status()
        .with_context(|| format!("启动命令失败: {}", command[0]))?;

    Ok(status.code().unwrap_or(1))
}
