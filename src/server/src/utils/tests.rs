#[cfg(test)]
mod path_tests {
    use super::super::path::*;
    use std::env;
    
    #[test]
    fn test_expand_env_vars() {
        // 保存当前环境变量
        let original_userprofile = env::var("USERPROFILE").ok();
        let original_appdata = env::var("APPDATA").ok();
        
        // 设置测试环境变量
        unsafe {
            env::set_var("USERPROFILE", "C:\\Users\\TestUser");
            env::set_var("APPDATA", "C:\\Users\\TestUser\\AppData\\Roaming");
        }
        
        // 测试环境变量替换
        let path_with_vars = "%USERPROFILE%\\Documents\\config.json";
        let expanded_path = expand_env_vars(path_with_vars);
        assert_eq!(expanded_path, "C:\\Users\\TestUser\\Documents\\config.json");
        
        let path_with_multiple_vars = "%APPDATA%\\Code\\%USERPROFILE%\\settings.json";
        let expanded_path = expand_env_vars(path_with_multiple_vars);
        assert_eq!(expanded_path, "C:\\Users\\TestUser\\AppData\\Roaming\\Code\\C:\\Users\\TestUser\\settings.json");
        
        // 测试不包含环境变量的路径
        let path_without_vars = "C:\\Program Files\\App\\config.json";
        let expanded_path = expand_env_vars(path_without_vars);
        assert_eq!(expanded_path, path_without_vars);
        
        // 恢复原始环境变量
        unsafe {
            if let Some(val) = original_userprofile {
                env::set_var("USERPROFILE", val);
            } else {
                env::remove_var("USERPROFILE");
            }
            
            if let Some(val) = original_appdata {
                env::set_var("APPDATA", val);
            } else {
                env::remove_var("APPDATA");
            }
        }
    }
    
    #[test]
    fn test_is_absolute_path() {
        // Windows 绝对路径
        assert!(is_absolute_path("C:\\Users\\Test"));
        assert!(is_absolute_path("D:\\Program Files\\App"));
        
        // 相对路径
        assert!(!is_absolute_path("Users\\Test"));
        assert!(!is_absolute_path(".\\config.json"));
        assert!(!is_absolute_path("..\\parent\\file.txt"));
    }
    
    #[test]
    fn test_to_absolute_path() {
        // 绝对路径应该保持不变
        let abs_path = "C:\\Users\\Test\\file.txt";
        let result = to_absolute_path(abs_path);
        assert_eq!(result.to_str().unwrap(), abs_path);
        
        // 相对路径应该转换为绝对路径
        // 注意：这个测试依赖于当前工作目录，可能不稳定
        let current_dir = env::current_dir().unwrap();
        let rel_path = "file.txt";
        let expected = current_dir.join(rel_path);
        let result = to_absolute_path(rel_path);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod command_tests {
    use super::super::command::*;
    
    #[test]
    fn test_execute_command() {
        // 测试简单的 echo 命令
        if cfg!(target_os = "windows") {
            let result = execute_command("echo Hello, World!");
            assert!(result.is_ok());
            let output = result.unwrap();
            assert!(output.contains("Hello, World!"));
        } else {
            let result = execute_command("echo 'Hello, World!'");
            assert!(result.is_ok());
            let output = result.unwrap();
            assert!(output.contains("Hello, World!"));
        }
    }
    
    #[test]
    fn test_execute_command_error() {
        // 测试不存在的命令
        let result = execute_command("non_existent_command_123");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_execute_powershell() {
        // 仅在 Windows 上测试 PowerShell
        if cfg!(target_os = "windows") {
            let result = execute_powershell("Write-Output 'Hello from PowerShell'");
            assert!(result.is_ok());
            let output = result.unwrap();
            assert!(output.contains("Hello from PowerShell"));
        }
    }
}
