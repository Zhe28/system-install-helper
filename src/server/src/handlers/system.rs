use actix_web::{web, HttpResponse, Responder, get};
use log::info;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_system_info)
        .service(get_system_status);
}

#[get("/system/info")]
async fn get_system_info() -> impl Responder {
    info!("获取系统信息");
    
    // 这里将来会获取实际的系统信息
    // 目前返回一些示例数据
    HttpResponse::Ok().json(serde_json::json!({
        "os": "Windows 11",
        "version": "22H2",
        "architecture": "x64",
        "hostname": "DESKTOP-PC",
        "username": "User",
        "cpu": {
            "model": "Intel Core i7",
            "cores": 8,
            "threads": 16
        },
        "memory": {
            "total": "16GB",
            "available": "8GB"
        },
        "disk": {
            "total": "512GB",
            "available": "256GB"
        }
    }))
}

#[get("/system/status")]
async fn get_system_status() -> impl Responder {
    info!("获取系统状态");
    
    // 这里将来会获取实际的系统状态
    // 目前返回一些示例数据
    HttpResponse::Ok().json(serde_json::json!({
        "cpu_usage": 25.5,
        "memory_usage": 45.2,
        "disk_usage": 50.0,
        "network": {
            "download": "1.2 MB/s",
            "upload": "0.5 MB/s"
        },
        "processes": 120,
        "uptime": "2 days, 5 hours"
    }))
}
