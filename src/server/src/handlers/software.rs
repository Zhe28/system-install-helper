use crate::models::software::{Software, SoftwareInstallRequest};
use crate::services::software_service::{SoftwareService, SoftwareServiceImpl};
use actix_web::{HttpResponse, Responder, get, post, web};
use log::info;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/software")
            .service(get_all_software)
            .service(get_software_by_id)
            .service(install_software)
            .service(get_install_status)
            .service(search_software)
            .service(edit_software),
    );
}

#[get("/")]
async fn get_all_software() -> impl Responder {
    info!("获取所有软件列表");

    // 从配置目录中读取所有软件信息
    // let software_list = software_config::load_all_software();
    let software_service = SoftwareServiceImpl::new();
    let software_list = software_service.get_all_software().await;

    match software_list {
        Ok(software_list) => HttpResponse::Ok().json(software_list),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/edit")]
async fn edit_software(request: web::Json<Software>) -> impl Responder {
    info!("编辑软件，ID: {}", request.id);

    // 调用服务层的编辑方法
    let service: SoftwareServiceImpl = SoftwareServiceImpl::new();
    let result: Result<String, anyhow::Error> = service.edit_software(&request).await;

    // 返回结果
    HttpResponse::Ok().body(result.unwrap())
}

#[get("/{id}")]
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

#[post("/install")]
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

#[get("/install/{task_id}")]
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

#[get("/search")]
async fn search_software(
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let search_term = query.get("q").cloned().unwrap_or_default();
    info!("搜索软件: {}", search_term);

    // 这里将来会实际搜索软件
    // 目前返回一些示例数据
    let software_list = vec![Software::new(
        "Visual Studio Code".to_string(),
        "1.85.0".to_string(),
        Some("轻量级代码编辑器".to_string()),
        "winget install Microsoft.VisualStudioCode".to_string(),
        crate::models::software::SoftwareCategory::Development,
    )];

    HttpResponse::Ok().json(software_list)
}
