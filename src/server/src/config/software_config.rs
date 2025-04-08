use std::fs;
use std::path::{Path, PathBuf};
use log::{info, error};
use crate::models::software::{
    Software, SoftwareCategory, SoftwareToml, SoftwareInfo, 
    Tags, Dependencies, Paths
};
use chrono::Utc;

/// 从TOML文件加载软件信息
pub fn load_software_from_toml(file_path: &Path) -> Result<Software, anyhow::Error> {
    let content = fs::read_to_string(file_path)?;
    let software_toml: SoftwareToml = toml::from_str(&content)?;
    
    // 解析软件类别
    let category = match software_toml.software.category.as_str() {
        "Development" => SoftwareCategory::Development,
        "Productivity" => SoftwareCategory::Productivity,
        "Utility" => SoftwareCategory::Utility,
        "Entertainment" => SoftwareCategory::Entertainment,
        "Communication" => SoftwareCategory::Communication,
        "Security" => SoftwareCategory::Security,
        "System" => SoftwareCategory::System,
        _ => SoftwareCategory::Other,
    };
    
    let now = Utc::now();
    
    // 创建Software对象
    let software = Software {
        id: software_toml.software.id,
        name: software_toml.software.name,
        version: software_toml.software.version,
        description: software_toml.software.description,
        install_command: software_toml.software.install_command,
        uninstall_command: software_toml.software.uninstall_command,
        category,
        tags: software_toml.tags.tags,
        dependencies: software_toml.dependencies.dependencies,
        install_path: software_toml.paths.install_path,
        config_files: software_toml.paths.config_files,
        created_at: now,
        updated_at: now,
    };
    
    Ok(software)
}

/// 从配置目录加载所有软件信息
pub fn load_all_software() -> Vec<Software> {
    let config_dir = Path::new("config/software");
    let mut software_list = Vec::new();
    
    if !config_dir.exists() {
        error!("软件配置目录不存在: {:?}", config_dir);
        return software_list;
    }
    
    match fs::read_dir(config_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                        info!("加载软件配置文件: {:?}", path);
                        match load_software_from_toml(&path) {
                            Ok(software) => {
                                software_list.push(software);
                            },
                            Err(err) => {
                                error!("加载软件配置文件失败 {:?}: {}", path, err);
                            }
                        }
                    }
                }
            }
        },
        Err(err) => {
            error!("读取软件配置目录失败: {}", err);
        }
    }
    
    info!("成功加载 {} 个软件配置", software_list.len());
    software_list
}
