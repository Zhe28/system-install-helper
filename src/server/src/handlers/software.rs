use actix_web::{web, HttpResponse, Responder, get, post, delete};
use crate::models::software::{Software, SoftwareInstallRequest};
use log::info;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_software)
        .service(get_software_by_id)
        .service(install_software)
        .service(get_install_status)
        .service(search_software);
}

#[get("/software")]
async fn get_all_software() -> impl Responder {
    info!("获取所有软件列表");
    
    // 这里将来会从数据库或文件中获取软件列表
    // 目前返回一些示例数据
    let software_list = vec![
        Software::new(
            "Visual Studio Code".to_string(),
            "1.85.0".to_string(),
            Some("轻量级代码编辑器".to_string()),
            "winget install Microsoft.VisualStudioCode".to_string(),
            crate::models::software::SoftwareCategory::Development,
        ),
        Software::new(
            "Google Chrome".to_string(),
            "120.0.6099.130".to_string(),
            Some("网页浏览器".to_string()),
            "winget install Google.Chrome".to_string(),
            crate::models::software::SoftwareCategory::Communication,
        ),
    ];
    
    HttpResponse::Ok().json(software_list)
}

#[get("/software/{id}")]
async fn get_software_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    info!("获取软件详情，ID: {}", id);
    
    // 这里将来会从数据库或文件中获取特定软件
    // 目前返回一个示例数据
    let software = Software::new(
        "Visual Studio Code".to_string(),
        "1.85.0".to_string(),
        Some("轻量级代码编辑器".to_string()),
        "winget install Microsoft.VisualStudioCode".to_string(),
        crate::models::software::SoftwareCategory::Development,
    );
    
    HttpResponse::Ok().json(software)
}

#[post("/software/install")]
async fn install_software(request: web::Json<SoftwareInstallRequest>) -> impl Responder {
    info!("安装软件请求: {:?}", request);
    
    // 这里将来会实际执行软件安装
    // 目前返回一个示例响应
    HttpResponse::Accepted().json(serde_json::json!({
        "task_id": uuid::Uuid::new_v4().to_string(),
        "status": "pending",
        "message": "软件安装任务已创建"
    }))
}

#[get("/software/install/{task_id}")]
async fn get_install_status(path: web::Path<String>) -> impl Responder {
    let task_id = path.into_inner();
    info!("获取安装状态，任务ID: {}", task_id);
    
    // 这里将来会查询实际的安装状态
    // 目前返回一个示例响应
    HttpResponse::Ok().json(serde_json::json!({
        "task_id": task_id,
        "status": "in_progress",
        "progress": 0.45,
        "message": "正在安装软件..."
    }))
}

#[get("/software/search")]
async fn search_software(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let search_term = query.get("q").cloned().unwrap_or_default();
    info!("搜索软件: {}", search_term);
    
    // 这里将来会实际搜索软件
    // 目前返回一些示例数据
    let software_list = vec![
        Software::new(
            "Visual Studio Code".to_string(),
            "1.85.0".to_string(),
            Some("轻量级代码编辑器".to_string()),
            "winget install Microsoft.VisualStudioCode".to_string(),
            crate::models::software::SoftwareCategory::Development,
        ),
    ];
    
    HttpResponse::Ok().json(software_list)
}
