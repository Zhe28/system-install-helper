use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use log::info;
use listenfd::ListenFd;

mod config;
mod handlers;
mod models;
mod services;
mod utils;

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
        },
        None => {
            info!("使用标准绑定方式: {}", bind_address);
            server.bind(&bind_address)?
        }
    };
    
    server.run().await
}
