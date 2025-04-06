use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub software: SoftwareConfig,
    pub config_files: ConfigFilesSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SoftwareConfig {
    pub repositories: Vec<String>,
    pub default_install_path: String,
    pub cache_dir: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFilesSettings {
    pub backup_dir: String,
    pub default_config_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            software: SoftwareConfig {
                repositories: vec![
                    "https://mirrors.tuna.tsinghua.edu.cn/winget/".to_string(),
                    "https://mirrors.ustc.edu.cn/winget/".to_string(),
                ],
                default_install_path: "C:\\Program Files".to_string(),
                cache_dir: ".\\cache".to_string(),
            },
            config_files: ConfigFilesSettings {
                backup_dir: ".\\backups".to_string(),
                default_config_path: "%USERPROFILE%".to_string(),
            },
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_path = "config.toml";
    
    if !Path::new(config_path).exists() {
        let default_config = Config::default();
        let toml_string = toml::to_string_pretty(&default_config)?;
        fs::write(config_path, toml_string)?;
        return Ok(default_config);
    }
    
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = "config.toml";
    let toml_string = toml::to_string_pretty(config)?;
    fs::write(config_path, toml_string)?;
    
    Ok(())
}

#[cfg(test)]
mod tests;
