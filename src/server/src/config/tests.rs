#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        // 验证默认配置值
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert!(config.software.repositories.len() >= 1);
        assert!(!config.software.default_install_path.is_empty());
        assert!(!config.software.cache_dir.is_empty());
        assert!(!config.config_files.backup_dir.is_empty());
        assert!(!config.config_files.default_config_path.is_empty());
    }
    
    #[test]
    fn test_save_and_load_config() {
        // 创建一个临时配置文件路径
        let temp_path = std::env::temp_dir().join("test_config.toml");
        let temp_path_str = temp_path.to_str().unwrap();
        
        // 创建自定义配置
        let mut config = Config::default();
        config.server.port = 9090;
        config.software.repositories = vec!["https://test-repo.com".to_string()];
        
        // 保存配置到临时文件
        let toml_string = toml::to_string_pretty(&config).unwrap();
        std::fs::write(&temp_path, toml_string).unwrap();
        
        // 从临时文件加载配置
        let config_content = std::fs::read_to_string(&temp_path).unwrap();
        let loaded_config: Config = toml::from_str(&config_content).unwrap();
        
        // 验证加载的配置
        assert_eq!(loaded_config.server.port, 9090);
        assert_eq!(loaded_config.software.repositories.len(), 1);
        assert_eq!(loaded_config.software.repositories[0], "https://test-repo.com");
        
        // 清理临时文件
        let _ = std::fs::remove_file(temp_path);
    }
}
