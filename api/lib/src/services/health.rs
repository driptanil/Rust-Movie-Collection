use actix_web::{ web, HttpResponse };

pub const API_VERSION: &str = "0.0.1";

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health));
}

#[tracing::instrument]
pub async fn get_health() -> HttpResponse {
    HttpResponse::Ok().append_header(("version", API_VERSION)).finish()
}
