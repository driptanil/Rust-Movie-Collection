use actix_web::web;

pub mod version_router;
pub mod movies_router;
pub mod health_router;
pub mod user_router;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/api")
            // .configure(version_router::init_routes)
            .configure(movies_router::router)
            // .configure(health_router::router)
            .configure(user_router::router)
    );
}
