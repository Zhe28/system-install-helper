use anyhow::Result;
use log::{info, error};
use std::process::{Command, Output};

/// 执行命令行命令
pub fn execute_command(command: &str) -> Result<String> {
    info!("执行命令: {}", command);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?
    };
    
    process_output(output)
}

/// 处理命令输出
fn process_output(output: Output) -> Result<String> {
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    } else {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        error!("命令执行失败: {}", error);
        Err(anyhow::anyhow!("命令执行失败: {}", error))
    }
}

/// 异步执行命令行命令
#[cfg(feature = "async")]
pub async fn execute_command_async(command: &str) -> Result<String> {
    use tokio::process::Command as TokioCommand;
    
    info!("异步执行命令: {}", command);
    
    let output = if cfg!(target_os = "windows") {
        TokioCommand::new("cmd")
            .args(&["/C", command])
            .output()
            .await?
    } else {
        TokioCommand::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .await?
    };
    
    process_output(output)
}

/// 执行 PowerShell 命令
pub fn execute_powershell(script: &str) -> Result<String> {
    info!("执行 PowerShell 脚本");
    
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(script)
        .output()?;
        
    process_output(output)
}

/// 执行 WMI 查询
pub fn execute_wmi_query(query: &str) -> Result<String> {
    let command = format!("wmic path {} /format:list", query);
    execute_command(&command)
}
