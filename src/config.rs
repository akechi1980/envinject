use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub env: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigStore {
    pub projects: BTreeMap<String, ProjectConfig>,
}

#[derive(Debug, Clone)]
pub struct ConfigManager {
    pub path: PathBuf,
}

impl ConfigManager {
    pub fn new_default() -> Result<Self> {
        let dirs = ProjectDirs::from("dev", "envinject", "envinject")
            .context("无法定位系统配置目录，请检查用户环境")?;

        let path = dirs.config_dir().join("config.json");
        Ok(Self { path })
    }

    pub fn load_store(&self) -> Result<ConfigStore> {
        if !self.path.exists() {
            return Ok(ConfigStore::default());
        }

        let text = fs::read_to_string(&self.path)
            .with_context(|| format!("读取配置文件失败: {}", self.path.display()))?;
        if text.trim().is_empty() {
            return Ok(ConfigStore::default());
        }

        let store: ConfigStore = serde_json::from_str(&text)
            .with_context(|| format!("解析配置文件失败: {}", self.path.display()))?;
        Ok(store)
    }

    pub fn save_store(&self, store: &ConfigStore) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("创建配置目录失败: {}", parent.display()))?;
        }

        let text = serde_json::to_string_pretty(store).context("序列化配置失败")?;
        fs::write(&self.path, text)
            .with_context(|| format!("写入配置文件失败: {}", self.path.display()))?;
        Ok(())
    }
}
