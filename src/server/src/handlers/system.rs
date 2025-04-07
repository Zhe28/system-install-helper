use actix_web::{web, HttpResponse, Responder, get};
use log::info;
use crate::services::system::{SystemService, SystemServiceImpl};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_system_info)
        .service(get_system_status);
}

#[get("/system/info")]
async fn get_system_info() -> impl Responder {
    info!("获取系统信息");
    
    let system_service = SystemServiceImpl::new();
    match system_service.get_system_info().await {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => {
            log::error!("获取系统信息失败: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("获取系统信息失败: {}", e)
            }))
        }
    }
}

#[get("/system/status")]
async fn get_system_status() -> impl Responder {
    info!("获取系统状态");
    
    let system_service = SystemServiceImpl::new();
    match system_service.get_system_status().await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => {
            log::error!("获取系统状态失败: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("获取系统状态失败: {}", e)
            }))
        }
    }
}
