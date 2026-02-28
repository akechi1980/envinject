mod config;
mod exec;
mod gui;

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::ConfigManager;

#[derive(Debug, Parser)]
#[command(
    name = "envinject",
    version,
    about = "按项目管理环境变量，并在运行命令时临时注入。"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 注入指定项目的环境变量并执行命令
    Run {
        project: String,
        #[arg(required = true, last = true, trailing_var_arg = true)]
        command: Vec<String>,
    },
    /// 打开图形配置编辑器
    Gui,
    /// 列出所有项目
    List,
    /// 显示指定项目的所有键值
    Show { project: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { project, command } => {
            let manager = ConfigManager::new_default()?;
            let store = manager.load_store()?;
            let envs = store
                .projects
                .get(&project)
                .map(|p| p.env.clone())
                .with_context(|| format!("项目不存在: {project}"))?;

            let code = exec::run_with_env(&command, envs)?;
            std::process::exit(code);
        }
        Commands::Gui => {
            gui::run_gui()?;
        }
        Commands::List => {
            let manager = ConfigManager::new_default()?;
            let store = manager.load_store()?;
            if store.projects.is_empty() {
                println!("暂无项目。可先运行 `envinject gui` 创建。");
            } else {
                for name in store.projects.keys() {
                    println!("{name}");
                }
            }
        }
        Commands::Show { project } => {
            let manager = ConfigManager::new_default()?;
            let store = manager.load_store()?;
            let envs = store
                .projects
                .get(&project)
                .map(|p| &p.env)
                .with_context(|| format!("项目不存在: {project}"))?;
            print_env_table(envs);
        }
    }

    Ok(())
}

fn print_env_table(envs: &BTreeMap<String, String>) {
    if envs.is_empty() {
        println!("该项目暂无环境变量。");
        return;
    }

    for (k, v) in envs {
        println!("{k}={v}");
    }
}
