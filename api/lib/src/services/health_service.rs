use actix_web::HttpResponse;

pub const API_VERSION: &str = "0.0.1";

#[tracing::instrument]
pub async fn get_health() -> HttpResponse {
    HttpResponse::Ok().append_header(("version", API_VERSION)).finish()
}
