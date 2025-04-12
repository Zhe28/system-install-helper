use actix_web::web;

mod config_files;
mod software;
mod system;

pub fn config_app(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .configure(config_files::config)
      .configure(system::config)
      .configure(software::config),
  );
}
