use crate::config::software_config;
use crate::{
    SOFTWARES,
    models::software::{InstallStatus, Software, SoftwareInstallStatus},
};
use anyhow::Result;
use async_trait::async_trait;
use log::{error, info};
use std::path::Path;
use std::process::Command;

#[async_trait]
pub trait SoftwareService {
    async fn get_all_software(&self) -> Result<Vec<Software>>;
    async fn get_software_by_id(&self, id: &str) -> Result<Option<Software>>;
    async fn search_software(&self, query: &str) -> Result<Vec<Software>>;
    async fn install_software(
        &self,
        software: &Software,
        install_path: Option<String>,
    ) -> Result<String>;
    async fn get_install_status(&self, task_id: &str) -> Result<SoftwareInstallStatus>;
    async fn edit_software(&self, software: &Software) -> Result<String>;
}

pub struct SoftwareServiceImpl {
    config_dir: String,
    // 这里可以添加数据存储、缓存等依赖
}

impl SoftwareServiceImpl {
    pub fn new() -> Self {
        SoftwareServiceImpl {
            config_dir: String::from("config/software"),
        }
    }

    // 执行命令行命令的辅助方法
    async fn execute_command(&self, command: &str) -> Result<String> {
        info!("执行命令: {}", command);

        // 在实际实现中，这里会使用 tokio::process::Command 异步执行
        // 目前使用同步版本作为示例
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", command]).output()?
        } else {
            Command::new("sh").arg("-c").arg(command).output()?
        };

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(result)
        } else {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            error!("命令执行失败: {}", error);
            Err(anyhow::anyhow!("命令执行失败: {}", error))
        }
    }
}

#[async_trait]
impl SoftwareService for SoftwareServiceImpl {
    async fn get_all_software(&self) -> Result<Vec<Software>> {
        // 在实际实现中，这里会从数据库或文件中获取软件列表
        let software_list = SOFTWARES.lock().unwrap();

        Ok(software_list.clone())
    }

    async fn edit_software(&self, software: &Software) -> Result<String> {
        // 通过软件 ID 找到软件
        let software = self.get_software_by_id(&software.id).await?;

        // 目前返回一个示例数据
        Ok("软件编辑成功".to_string())
    }

    /**
     * 通过软件 ID 获取软件
     * @param id 软件 ID
     * @return 软件信息
     */
    async fn get_software_by_id(&self, id: &str) -> Result<Option<Software>> {
        let config_dir = Path::new(&self.config_dir);

        if !config_dir.exists() {
            error!("软件配置目录不存在: {:?}", config_dir);
            return Ok(None);
        }

        // 直接构造文件路径
        let file_path = config_dir.join(format!("{}.toml", id));

        // 检查文件是否存在
        if !file_path.exists() {
            info!("软件配置文件不存在: {:?}", file_path);
            return Ok(None);
        }

        // 加载软件配置
        match software_config::load_software_from_toml(&file_path) {
            Ok(software) => {
                // 验证 ID 是否匹配
                if software.id == id {
                    info!("成功加载软件配置: ID={}", id);
                    return Ok(Some(software));
                } else {
                    error!(
                        "文件名与软件 ID 不匹配: 文件名为 '{}' 但软件 ID 为 '{}'",
                        id, software.id
                    );
                    return Ok(None);
                }
            }
            Err(err) => {
                error!("加载软件配置文件失败 {:?}: {}", file_path, err);
                return Ok(None);
            }
        }
    }

    /**
     * 搜索软件
     * @param query 搜索关键词
     * @return 软件列表
     */
    async fn search_software(&self, query: &str) -> Result<Vec<Software>> {
        // 在实际实现中，这里会搜索软件
        // 目前返回一些示例数据
        info!("搜索软件: {}", query);

        let software_list = vec![Software::new(
            "Visual Studio Code".to_string(),
            "1.85.0".to_string(),
            Some("轻量级代码编辑器".to_string()),
            "winget install Microsoft.VisualStudioCode".to_string(),
            crate::models::software::SoftwareCategory::Development,
        )];

        Ok(software_list)
    }

    /**
     * 安装软件
     * @param software 软件信息
     * @param install_path 自定义安装路径
     * @return 安装任务 ID
     */
    async fn install_software(
        &self,
        software: &Software,
        install_path: Option<String>,
    ) -> Result<String> {
        // 生成任务ID
        let task_id = uuid::Uuid::new_v4().to_string();

        // 构建安装命令
        let mut install_command = software.install_command.clone();

        // 如果提供了自定义安装路径，添加到命令中
        if let Some(path) = install_path {
            // 这里需要根据实际的安装程序调整参数
            // 例如，对于 winget，可能需要使用 --location 参数
            install_command = format!("{} --location \"{}\"", install_command, path);
        }

        // 在实际实现中，这里会异步执行安装命令
        // 并更新安装状态
        // 目前只是记录日志
        info!("开始安装软件: {}, 命令: {}", software.name, install_command);

        // 返回任务ID，用于后续查询安装状态
        Ok(task_id)
    }

    /**
     * 获取安装状态
     * @param task_id 安装任务 ID
     * @return 安装状态
     */
    async fn get_install_status(&self, task_id: &str) -> Result<SoftwareInstallStatus> {
        // 在实际实现中，这里会查询实际的安装状态
        // 目前返回一个示例状态
        let status = SoftwareInstallStatus {
            id: task_id.to_string(),
            software_id: "sample-software-id".to_string(),
            status: InstallStatus::Installing,
            progress: 0.45,
            message: Some("正在安装软件...".to_string()),
            started_at: chrono::Utc::now(),
            completed_at: None,
        };

        Ok(status)
    }
}
