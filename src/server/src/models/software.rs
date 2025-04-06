use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Software {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub install_command: String,
    pub uninstall_command: Option<String>,
    pub category: SoftwareCategory,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub install_path: Option<String>,
    pub config_files: Vec<String>,
    pub created_at: DateTime<Utc>,
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
