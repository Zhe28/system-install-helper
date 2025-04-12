use crate::models::software::Software;
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware};
use crate::config::software_config::load_all_software;
use listenfd::ListenFd;
use log::info;
use once_cell::sync::Lazy;
use std::sync::Mutex;

mod config;
mod handlers;
mod models;
mod services;
mod utils;

static VERSION: &str = env!("CARGO_PKG_VERSION");

// 定义全局的软件列表，防止后续重复加载
static SOFTWARES: Lazy<Mutex<Vec<Software>>> = Lazy::new(|| Mutex::new(load_all_software()));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("系统安装助手后台服务启动中...");

    let config = config::load_config().expect("无法加载配置");
    let bind_address = format!("{}:{}", config.server.host, config.server.port);

    info!("服务器监听地址: {}", bind_address);

    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(handlers::config_app)
    });

    // 尝试使用 socket 监听器进行热重载
    let mut listenfd = ListenFd::from_env();
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => {
            info!("使用系统提供的监听器进行热重载");
            server.listen(listener)?
        }
        None => {
            info!("使用标准绑定方式: {}", bind_address);
            server.bind(&bind_address)?
        }
    };

    server.run().await
}
