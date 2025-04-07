use anyhow::Result;
use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};
use num_cpus;
use sysinfo::{System, Disks, Networks};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub version: String,
    pub architecture: String,
    pub hostname: String,
    pub username: String,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
    pub network: NetworkInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub threads: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: String,
    pub available: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub total: String,
    pub available: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub mac_address: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network: NetworkStatus,
    pub processes: u32,
    pub uptime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub download: String,
    pub upload: String,
}

#[async_trait]
pub trait SystemService {
    async fn get_system_info(&self) -> Result<SystemInfo>;
    async fn get_system_status(&self) -> Result<SystemStatus>;
}

pub struct SystemServiceImpl {
    // 这里可以添加依赖
}

impl SystemServiceImpl {
    pub fn new() -> Self {
        SystemServiceImpl {}
    }
    
    // 将字节转换为人类可读的大小
    fn format_bytes(&self, bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;
        
        if bytes >= TB {
            format!("{:.2} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
    
    // 格式化时间为可读形式
    fn format_duration(&self, seconds: u64) -> String {
        let days = seconds / (24 * 3600);
        let hours = (seconds % (24 * 3600)) / 3600;
        let minutes = (seconds % 3600) / 60;
        
        if days > 0 {
            format!("{} 天, {} 小时, {} 分钟", days, hours, minutes)
        } else if hours > 0 {
            format!("{} 小时, {} 分钟", hours, minutes)
        } else {
            format!("{} 分钟", minutes)
        }
    }
    
    // 获取网络接口信息 - 改进版本
    fn get_network_interfaces(&self) -> Vec<NetworkInterface> {
        let mut interfaces = Vec::new();
        
        // 使用 PowerShell 获取更详细的网络信息
        let ps_command = r#"
        Get-NetAdapter | ForEach-Object {
            $adapter = $_
            $config = Get-NetIPConfiguration -InterfaceIndex $adapter.ifIndex
            $ipAddresses = @()
            
            # 获取 IPv4 地址
            $ipv4 = $config.IPv4Address
            if ($ipv4) {
                foreach ($ip in $ipv4) {
                    $ipAddresses += $ip.IPAddress
                }
            }
            
            # 获取 IPv6 地址
            $ipv6 = $config.IPv6Address
            if ($ipv6) {
                foreach ($ip in $ipv6) {
                    $ipAddresses += $ip.IPAddress
                }
            }
            
            # 输出结果
            [PSCustomObject]@{
                Name = $adapter.Name
                Status = $adapter.Status
                MacAddress = $adapter.MacAddress
                IPAddresses = $ipAddresses -join ','
            } | ConvertTo-Json
        }
        "#;
        
        // 执行 PowerShell 命令
        let output = Command::new("powershell")
            .args(&["-Command", ps_command])
            .output();
            
        if let Ok(output) = output {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                // 解析每个接口的 JSON 输出
                for line in output_str.lines() {
                    let line = line.trim();
                    if line.starts_with("{") && line.ends_with("}") {
                        // 尝试解析 JSON
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                            let name = json["Name"].as_str().unwrap_or("未知").to_string();
                            let status = json["Status"].as_str().unwrap_or("未知").to_string();
                            let mac = json["MacAddress"].as_str().unwrap_or("未知").to_string();
                            let ip_str = json["IPAddresses"].as_str().unwrap_or("").to_string();
                            
                            // 解析 IP 地址列表
                            let ip_addresses: Vec<String> = if !ip_str.is_empty() {
                                ip_str.split(',').map(|s| s.trim().to_string()).collect()
                            } else {
                                Vec::new()
                            };
                            
                            // 创建网络接口对象
                            let interface = NetworkInterface {
                                name,
                                ip_addresses,
                                mac_address: mac,
                                status,
                            };
                            
                            interfaces.push(interface);
                        }
                    }
                }
            }
        }
        
        // 如果 PowerShell 方法失败，尝试使用 ipconfig 命令作为备选方案
        if interfaces.is_empty() {
            info!("PowerShell 获取网络信息失败，尝试使用 ipconfig 命令");
            
            if let Ok(output) = Command::new("ipconfig")
                .arg("/all")
                .output() {
                
                if let Ok(output_str) = String::from_utf8(output.stdout) {
                    let mut current_interface: Option<NetworkInterface> = None;
                    
                    for line in output_str.lines() {
                        let line = line.trim();
                        
                        // 检测新的网络接口部分
                        if !line.is_empty() && !line.starts_with(" ") && line.contains(":") {
                            // 保存之前的接口
                            if let Some(interface) = current_interface.take() {
                                if !interface.name.is_empty() {
                                    interfaces.push(interface);
                                }
                            }
                            
                            // 创建新的接口
                            let name = line.split(':').next().unwrap_or("").trim().to_string();
                            if !name.is_empty() {
                                current_interface = Some(NetworkInterface {
                                    name,
                                    ip_addresses: Vec::new(),
                                    mac_address: "未知".to_string(),
                                    status: "未知".to_string(),
                                });
                            }
                        } else if let Some(ref mut interface) = current_interface {
                            // IPv4 地址
                            if line.contains("IPv4") && line.contains(":") {
                                let parts: Vec<&str> = line.split(':').collect();
                                if parts.len() > 1 {
                                    let ip = parts[1].trim().to_string();
                                    if !ip.is_empty() {
                                        interface.ip_addresses.push(ip);
                                    }
                                }
                            }
                            // MAC 地址
                            else if (line.contains("Physical Address") || line.contains("物理地址")) && line.contains(":") {
                                let parts: Vec<&str> = line.split(':').collect();
                                if parts.len() > 1 {
                                    interface.mac_address = parts[1].trim().to_string();
                                }
                            }
                            // 连接状态
                            else if (line.contains("Media State") || line.contains("媒体状态")) && line.contains(":") {
                                let status = if line.contains("Connected") || line.contains("已连接") {
                                    "已连接".to_string()
                                } else {
                                    "已断开连接".to_string()
                                };
                                interface.status = status;
                            }
                        }
                    }
                    
                    // 添加最后一个接口
                    if let Some(interface) = current_interface {
                        if !interface.name.is_empty() {
                            interfaces.push(interface);
                        }
                    }
                }
            }
        }
        
        // 如果仍然没有找到任何接口，添加一个默认接口
        if interfaces.is_empty() {
            interfaces.push(NetworkInterface {
                name: "默认网络接口".to_string(),
                ip_addresses: vec!["无法获取IP地址".to_string()],
                mac_address: "无法获取MAC地址".to_string(),
                status: "未知".to_string(),
            });
        }
        
        interfaces
    }
}

#[async_trait]
impl SystemService for SystemServiceImpl {
    async fn get_system_info(&self) -> Result<SystemInfo> {
        info!("获取系统信息");
        
        // 初始化系统信息
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // 获取操作系统信息
        let os_name = System::name().unwrap_or_else(|| "未知".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "未知".to_string());
        
        // 获取架构信息
        let architecture = std::env::consts::ARCH.to_string();
        
        // 获取主机名和用户名
        let hostname = System::host_name().unwrap_or_else(|| "未知".to_string());
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "未知".to_string());
        
        // 获取 CPU 信息
        let cpu_model = if let Some(cpu) = sys.cpus().first() {
            cpu.brand().to_string()
        } else {
            "未知".to_string()
        };
        let cpu_cores = num_cpus::get_physical() as u32;
        let cpu_threads = num_cpus::get() as u32;
        
        // 获取内存信息
        let total_memory = sys.total_memory();
        let available_memory = sys.available_memory();
        
        // 获取磁盘信息
        let mut total_disk: u64 = 0;
        let mut available_disk: u64 = 0;
        
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            total_disk += disk.total_space();
            available_disk += disk.available_space();
        }
        
        // 获取网络接口信息
        let network_interfaces = self.get_network_interfaces();
        
        // 构建系统信息
        let system_info = SystemInfo {
            os: os_name,
            version: os_version,
            architecture,
            hostname,
            username,
            cpu: CpuInfo {
                model: cpu_model,
                cores: cpu_cores,
                threads: cpu_threads,
            },
            memory: MemoryInfo {
                total: self.format_bytes(total_memory),
                available: self.format_bytes(available_memory),
            },
            disk: DiskInfo {
                total: self.format_bytes(total_disk),
                available: self.format_bytes(available_disk),
            },
            network: NetworkInfo {
                interfaces: network_interfaces,
            },
        };
        
        Ok(system_info)
    }
    
    async fn get_system_status(&self) -> Result<SystemStatus> {
        info!("获取系统状态");
        
        // 初始化系统信息
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // 计算 CPU 使用率 - 简化版本
        let cpu_usage = sys.global_cpu_usage();
        
        // 计算内存使用率
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let memory_usage = if total_memory > 0 {
            100.0 * (used_memory as f32 / total_memory as f32)
        } else {
            0.0
        };
        
        // 计算磁盘使用率
        let mut total_disk: u64 = 0;
        let mut used_disk: u64 = 0;
        
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            total_disk += disk.total_space();
            used_disk += disk.total_space() - disk.available_space();
        }
        
        let disk_usage = if total_disk > 0 {
            100.0 * (used_disk as f32 / total_disk as f32)
        } else {
            0.0
        };
        
        // 网络使用情况 - 简化版本
        let network_download = "获取中...".to_string();
        let network_upload = "获取中...".to_string();
        
        // 获取进程数量
        let processes_count = sys.processes().len() as u32;
        
        // 获取系统运行时间
        let uptime_seconds = System::uptime();
        let uptime = self.format_duration(uptime_seconds);
        
        // 构建系统状态
        let system_status = SystemStatus {
            cpu_usage,
            memory_usage,
            disk_usage,
            network: NetworkStatus {
                download: network_download,
                upload: network_upload,
            },
            processes: processes_count,
            uptime,
        };
        
        Ok(system_status)
    }
}
