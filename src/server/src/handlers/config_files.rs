use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use crate::models::config_file::{ConfigFile, ConfigFileRequest};
use log::info;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_config_files)
        .service(get_config_file_by_id)
        .service(create_config_file)
        .service(update_config_file)
        .service(delete_config_file)
        .service(deploy_config_file);
}

#[get("/config-files")]
async fn get_all_config_files() -> impl Responder {
    info!("获取所有配置文件");
    
    // 这里将来会从数据库或文件中获取配置文件列表
    // 目前返回一些示例数据
    let config_files = vec![
        ConfigFile::new(
            "VSCode 设置".to_string(),
            "./configs/vscode/settings.json".to_string(),
            "%APPDATA%/Code/User/settings.json".to_string(),
            Some("vscode-id".to_string()),
            false,
        ),
        ConfigFile::new(
            "Git 配置".to_string(),
            "./configs/git/.gitconfig".to_string(),
            "%USERPROFILE%/.gitconfig".to_string(),
            Some("git-id".to_string()),
            false,
        ),
    ];
    
    HttpResponse::Ok().json(config_files)
}

#[get("/config-files/{id}")]
async fn get_config_file_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    info!("获取配置文件详情，ID: {}", id);
    
    // 这里将来会从数据库或文件中获取特定配置文件
    // 目前返回一个示例数据
    let config_file = ConfigFile::new(
        "VSCode 设置".to_string(),
        "./configs/vscode/settings.json".to_string(),
        "%APPDATA%/Code/User/settings.json".to_string(),
        Some("vscode-id".to_string()),
        false,
    );
    
    HttpResponse::Ok().json(config_file)
}

#[post("/config-files")]
async fn create_config_file(request: web::Json<ConfigFileRequest>) -> impl Responder {
    info!("创建配置文件: {:?}", request);
    
    // 这里将来会实际创建配置文件
    // 目前返回一个示例响应
    let config_file = ConfigFile::new(
        request.name.clone(),
        request.source_path.clone(),
        request.target_path.clone(),
        request.software_id.clone(),
        request.is_directory,
    );
    
    HttpResponse::Created().json(config_file)
}

#[put("/config-files/{id}")]
async fn update_config_file(
    path: web::Path<String>,
    request: web::Json<ConfigFileRequest>,
) -> impl Responder {
    let id = path.into_inner();
    info!("更新配置文件，ID: {}, 数据: {:?}", id, request);
    
    // 这里将来会实际更新配置文件
    // 目前返回一个示例响应
    let mut config_file = ConfigFile::new(
        request.name.clone(),
        request.source_path.clone(),
        request.target_path.clone(),
        request.software_id.clone(),
        request.is_directory,
    );
    config_file.id = id;
    
    HttpResponse::Ok().json(config_file)
}

#[delete("/config-files/{id}")]
async fn delete_config_file(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    info!("删除配置文件，ID: {}", id);
    
    // 这里将来会实际删除配置文件
    // 目前返回一个示例响应
    HttpResponse::NoContent().finish()
}

#[post("/config-files/{id}/deploy")]
async fn deploy_config_file(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    info!("部署配置文件，ID: {}", id);
    
    // 这里将来会实际部署配置文件
    // 目前返回一个示例响应
    HttpResponse::Accepted().json(serde_json::json!({
        "task_id": uuid::Uuid::new_v4().to_string(),
        "status": "pending",
        "message": "配置文件部署任务已创建"
    }))
}
