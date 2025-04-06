use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub source_path: String,
    pub target_path: String,
    pub software_id: Option<String>,
    pub is_directory: bool,
    pub backup_on_install: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFileRequest {
    pub name: String,
    pub description: Option<String>,
    pub source_path: String,
    pub target_path: String,
    pub software_id: Option<String>,
    pub is_directory: bool,
    pub backup_on_install: bool,
}

impl ConfigFile {
    pub fn new(
        name: String,
        source_path: String,
        target_path: String,
        software_id: Option<String>,
        is_directory: bool,
    ) -> Self {
        let now = Utc::now();
        
        ConfigFile {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            source_path,
            target_path,
            software_id,
            is_directory,
            backup_on_install: true,
            created_at: now,
            updated_at: now,
        }
    }
}
