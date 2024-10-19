use actix_web::{ get, web, HttpResponse };
use crate::repositories::AppRepository;

type Repository = web::Data<Box<dyn AppRepository>>;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/version").service(get));
}

#[utoipa::path(context_path = "/api/version")]
#[get("")]
pub async fn get(repo: Repository) -> HttpResponse {
    tracing::info!("Getting version");

    match repo.get_version().await {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
