#[cfg(test)]
mod software_tests {
    use crate::models::software::*;
    
    #[test]
    fn test_software_new() {
        let name = "Test Software".to_string();
        let version = "1.0.0".to_string();
        let description = Some("Test Description".to_string());
        let install_command = "winget install TestSoftware".to_string();
        let category = SoftwareCategory::Development;
        
        let software = Software::new(
            name.clone(),
            version.clone(),
            description.clone(),
            install_command.clone(),
            category.clone(),
        );
        
        // 验证创建的软件对象
        assert_eq!(software.name, name);
        assert_eq!(software.version, version);
        assert_eq!(software.description, description);
        assert_eq!(software.install_command, install_command);
        assert_eq!(software.category, category);
        assert!(software.id.len() > 0); // UUID 不为空
        assert!(software.tags.is_empty());
        assert!(software.dependencies.is_empty());
        assert_eq!(software.install_path, None);
        assert!(software.config_files.is_empty());
        
        // 验证创建时间和更新时间
        assert!(software.created_at <= chrono::Utc::now());
        assert!(software.updated_at <= chrono::Utc::now());
        assert_eq!(software.created_at, software.updated_at);
    }
    
    #[test]
    fn test_software_category_serialization() {
        // 测试 SoftwareCategory 的序列化和反序列化
        let categories = vec![
            SoftwareCategory::Development,
            SoftwareCategory::Productivity,
            SoftwareCategory::Utility,
            SoftwareCategory::Entertainment,
            SoftwareCategory::Communication,
            SoftwareCategory::Security,
            SoftwareCategory::System,
            SoftwareCategory::Other,
        ];
        
        for category in categories {
            let serialized = serde_json::to_string(&category).unwrap();
            let deserialized: SoftwareCategory = serde_json::from_str(&serialized).unwrap();
            assert_eq!(category, deserialized);
        }
    }
    
    #[test]
    fn test_install_status_serialization() {
        // 测试 InstallStatus 的序列化和反序列化
        let statuses = vec![
            InstallStatus::Pending,
            InstallStatus::Installing,
            InstallStatus::Completed,
            InstallStatus::Failed,
        ];
        
        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: InstallStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }
}

#[cfg(test)]
mod config_file_tests {
    use crate::models::config_file::*;
    
    #[test]
    fn test_config_file_new() {
        let name = "Test Config".to_string();
        let source_path = "./test/source.json".to_string();
        let target_path = "%USERPROFILE%/test/target.json".to_string();
        let software_id = Some("test-software-id".to_string());
        let is_directory = false;
        
        let config_file = ConfigFile::new(
            name.clone(),
            source_path.clone(),
            target_path.clone(),
            software_id.clone(),
            is_directory,
        );
        
        // 验证创建的配置文件对象
        assert_eq!(config_file.name, name);
        assert_eq!(config_file.source_path, source_path);
        assert_eq!(config_file.target_path, target_path);
        assert_eq!(config_file.software_id, software_id);
        assert_eq!(config_file.is_directory, is_directory);
        assert!(config_file.id.len() > 0); // UUID 不为空
        assert_eq!(config_file.description, None);
        assert!(config_file.backup_on_install);
        
        // 验证创建时间和更新时间
        assert!(config_file.created_at <= chrono::Utc::now());
        assert!(config_file.updated_at <= chrono::Utc::now());
        assert_eq!(config_file.created_at, config_file.updated_at);
    }
}
