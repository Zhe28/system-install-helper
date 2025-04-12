use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Software {
    // 唯一ID
    pub id: String,
    // 软件名称
    pub name: String,
    // 软件版本
    pub version: String,
    // 软件描述
    pub description: Option<String>,
    // 安装命令
    pub install_command: String,
    // 软件卸载命令
    pub uninstall_command: Option<String>,
    // 软件类别
    pub category: SoftwareCategory,
    // 标签
    pub tags: Vec<String>,
    // 依赖项
    pub dependencies: Vec<String>,
    // 安装路径
    pub install_path: Option<String>,
    // 配置文件
    pub config_files: Vec<String>,
    // 创建时间
    pub created_at: DateTime<Utc>,
    // 更新时间
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SoftwareCategory {
    Development,
    Productivity,
    Utility,
    Entertainment,
    Communication,
    Security,
    System,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareInstallRequest {
    pub software_ids: Vec<String>,
    pub custom_install_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareInstallStatus {
    pub id: String,
    pub software_id: String,
    pub status: InstallStatus,
    pub progress: f32,
    pub message: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InstallStatus {
    Pending,
    Installing,
    Completed,
    Failed,
}

// TOML配置文件相关结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareToml {
    pub software: SoftwareInfo,
    pub tags: Tags,
    pub dependencies: Dependencies,
    pub paths: Paths,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub install_command: String,
    pub uninstall_command: Option<String>,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependencies {
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paths {
    pub install_path: Option<String>,
    pub config_files: Vec<String>,
}

impl Software {
    pub fn new(
        name: String,
        version: String,
        description: Option<String>,
        install_command: String,
        category: SoftwareCategory,
    ) -> Self {
        let now = Utc::now();
        
        Software {
            id: Uuid::new_v4().to_string(),
            name,
            version,
            description,
            install_command,
            uninstall_command: None,
            category,
            tags: Vec::new(),
            dependencies: Vec::new(),
            install_path: None,
            config_files: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
