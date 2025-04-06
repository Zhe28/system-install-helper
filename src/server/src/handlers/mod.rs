use actix_web::web;

mod software;
mod config_files;
mod system;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(software::config)
            .configure(config_files::config)
            .configure(system::config)
    );
}
