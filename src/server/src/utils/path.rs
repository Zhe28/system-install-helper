use std::path::{Path, PathBuf};
use std::env;

/// 解析路径中的环境变量
pub fn expand_env_vars(path: &str) -> String {
    let mut result = path.to_string();
    
    // 替换常见的 Windows 环境变量
    if result.contains("%USERPROFILE%") {
        if let Ok(userprofile) = env::var("USERPROFILE") {
            result = result.replace("%USERPROFILE%", &userprofile);
        }
    }
    
    if result.contains("%APPDATA%") {
        if let Ok(appdata) = env::var("APPDATA") {
            result = result.replace("%APPDATA%", &appdata);
        }
    }
    
    if result.contains("%LOCALAPPDATA%") {
        if let Ok(localappdata) = env::var("LOCALAPPDATA") {
            result = result.replace("%LOCALAPPDATA%", &localappdata);
        }
    }
    
    if result.contains("%PROGRAMFILES%") {
        if let Ok(programfiles) = env::var("PROGRAMFILES") {
            result = result.replace("%PROGRAMFILES%", &programfiles);
        }
    }
    
    if result.contains("%PROGRAMFILES(X86)%") {
        if let Ok(programfilesx86) = env::var("PROGRAMFILES(X86)") {
            result = result.replace("%PROGRAMFILES(X86)%", &programfilesx86);
        }
    }
    
    result
}

/// 确保目录存在，如果不存在则创建
pub fn ensure_dir_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 获取相对于应用程序的绝对路径
pub fn get_absolute_path(relative_path: &str) -> PathBuf {
    let base_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    base_dir.join(relative_path)
}

/// 检查路径是否为绝对路径
pub fn is_absolute_path(path: &str) -> bool {
    Path::new(path).is_absolute()
}

/// 将路径转换为绝对路径
pub fn to_absolute_path(path: &str) -> PathBuf {
    let path_obj = Path::new(path);
    
    if path_obj.is_absolute() {
        path_obj.to_path_buf()
    } else {
        get_absolute_path(path)
    }
}
