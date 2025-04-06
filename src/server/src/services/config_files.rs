use anyhow::Result;
use async_trait::async_trait;
use log::{info, error};
use std::fs;
use std::path::Path;
use crate::models::config_file::ConfigFile;

#[async_trait]
pub trait ConfigFileService {
    async fn get_all_config_files(&self) -> Result<Vec<ConfigFile>>;
    async fn get_config_file_by_id(&self, id: &str) -> Result<Option<ConfigFile>>;
    async fn create_config_file(&self, config_file: ConfigFile) -> Result<ConfigFile>;
    async fn update_config_file(&self, id: &str, config_file: ConfigFile) -> Result<Option<ConfigFile>>;
    async fn delete_config_file(&self, id: &str) -> Result<bool>;
    async fn deploy_config_file(&self, id: &str) -> Result<String>;
}

pub struct ConfigFileServiceImpl {
    // 这里可以添加数据存储、缓存等依赖
    backup_dir: String,
}

impl ConfigFileServiceImpl {
    pub fn new(backup_dir: String) -> Self {
        ConfigFileServiceImpl {
            backup_dir,
        }
    }
    
    // 创建备份的辅助方法
    async fn backup_file(&self, file_path: &str) -> Result<String> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Ok("文件不存在，无需备份".to_string());
        }
        
        let file_name = path.file_name()
            .ok_or_else(|| anyhow::anyhow!("无法获取文件名"))?
            .to_string_lossy();
            
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_file_name = format!("{}_{}", file_name, timestamp);
        let backup_path = Path::new(&self.backup_dir).join(backup_file_name);
        
        // 确保备份目录存在
        if let Some(parent) = backup_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 复制文件到备份位置
        fs::copy(path, &backup_path)?;
        
        info!("已备份文件 {} 到 {}", file_path, backup_path.display());
        
        Ok(backup_path.to_string_lossy().to_string())
    }
    
    // 部署配置文件的辅助方法
    async fn copy_file(&self, source: &str, target: &str, backup: bool) -> Result<()> {
        let source_path = Path::new(source);
        let target_path = Path::new(target);
        
        if !source_path.exists() {
            return Err(anyhow::anyhow!("源文件不存在: {}", source));
        }
        
        // 如果目标文件存在且需要备份
        if target_path.exists() && backup {
            self.backup_file(target).await?;
        }
        
        // 确保目标目录存在
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 复制文件到目标位置
        fs::copy(source_path, target_path)?;
        
        info!("已复制文件 {} 到 {}", source, target);
        
        Ok(())
    }
}

#[async_trait]
impl ConfigFileService for ConfigFileServiceImpl {
    async fn get_all_config_files(&self) -> Result<Vec<ConfigFile>> {
        // 在实际实现中，这里会从数据库或文件中获取配置文件列表
        // 目前返回一些示例数据
        let config_files = vec![
            ConfigFile::new(
                "VSCode 设置".to_string(),
                "./configs/vscode/settings.json".to_string(),
                "%APPDATA%/Code/User/settings.json".to_string(),
                Some("vscode-id".to_string()),
                false,
            ),
            ConfigFile::new(
                "Git 配置".to_string(),
                "./configs/git/.gitconfig".to_string(),
                "%USERPROFILE%/.gitconfig".to_string(),
                Some("git-id".to_string()),
                false,
            ),
        ];
        
        Ok(config_files)
    }
    
    async fn get_config_file_by_id(&self, id: &str) -> Result<Option<ConfigFile>> {
        // 在实际实现中，这里会从数据库或文件中获取特定配置文件
        // 目前返回一个示例数据
        let config_file = ConfigFile::new(
            "VSCode 设置".to_string(),
            "./configs/vscode/settings.json".to_string(),
            "%APPDATA%/Code/User/settings.json".to_string(),
            Some("vscode-id".to_string()),
            false,
        );
        
        Ok(Some(config_file))
    }
    
    async fn create_config_file(&self, config_file: ConfigFile) -> Result<ConfigFile> {
        // 在实际实现中，这里会将配置文件保存到数据库或文件中
        // 目前只是记录日志并返回原始数据
        info!("创建配置文件: {:?}", config_file);
        
        Ok(config_file)
    }
    
    async fn update_config_file(&self, id: &str, mut config_file: ConfigFile) -> Result<Option<ConfigFile>> {
        // 在实际实现中，这里会更新数据库或文件中的配置文件
        // 目前只是记录日志并返回更新后的数据
        info!("更新配置文件，ID: {}, 数据: {:?}", id, config_file);
        
        config_file.id = id.to_string();
        config_file.updated_at = chrono::Utc::now();
        
        Ok(Some(config_file))
    }
    
    async fn delete_config_file(&self, id: &str) -> Result<bool> {
        // 在实际实现中，这里会从数据库或文件中删除配置文件
        // 目前只是记录日志并返回成功
        info!("删除配置文件，ID: {}", id);
        
        Ok(true)
    }
    
    async fn deploy_config_file(&self, id: &str) -> Result<String> {
        // 生成任务ID
        let task_id = uuid::Uuid::new_v4().to_string();
        
        // 在实际实现中，这里会获取配置文件并部署
        // 目前只是记录日志
        info!("开始部署配置文件，ID: {}, 任务ID: {}", id, task_id);
        
        // 模拟获取配置文件
        let config_file = self.get_config_file_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("配置文件不存在"))?;
            
        // 解析路径中的环境变量
        let target_path = config_file.target_path.replace("%USERPROFILE%", &std::env::var("USERPROFILE").unwrap_or_default())
            .replace("%APPDATA%", &std::env::var("APPDATA").unwrap_or_default());
            
        // 部署配置文件
        self.copy_file(&config_file.source_path, &target_path, config_file.backup_on_install).await?;
        
        // 返回任务ID，用于后续查询部署状态
        Ok(task_id)
    }
}
