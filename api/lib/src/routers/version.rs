use actix_web::{ web, HttpResponse };
use crate::repositories::version::VersionRepository as Repository;

pub fn router<R: Repository>(cfg: &mut web::ServiceConfig) {
    cfg.route("/version", web::get().to(get::<R>));
}

pub async fn get<R: Repository>(repo: web::Data<R>) -> HttpResponse {
    tracing::info!("Getting version");

    match repo.get_version().await {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
