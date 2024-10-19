use actix_web::web;

pub mod version;
pub mod movies;
pub mod health;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/api")
            .configure(version::init_routes)
            .configure(movies::init_routes)
            .configure(health::router)
    );
}
