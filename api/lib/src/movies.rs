use actix_web::{ web, HttpResponse };
use sqlx::PgPool;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/v1/movies")
            .route("", web::get().to(get_all))
            .route("/{movie_id}", web::get().to(get_by_id))
            .route("", web::post().to(post))
            .route("", web::put().to(put))
            .route("/{movie_id}", web::delete().to(delete))
    );
}

#[tracing::instrument]
pub async fn get_all(db: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Getting all movies");

    HttpResponse::Ok().finish()
}

#[tracing::instrument]
pub async fn get_by_id(db: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("Getting movie by id");

    HttpResponse::Ok().finish()
}

#[tracing::instrument]
async fn post() -> HttpResponse {
    tracing::info!("Creating a new movie");

    HttpResponse::Ok().finish()
}

#[tracing::instrument]
async fn put() -> HttpResponse {
    tracing::info!("Updating a movie");

    HttpResponse::Ok().finish()
}

#[tracing::instrument]
async fn delete() -> HttpResponse {
    tracing::info!("Deleting a movie");

    HttpResponse::Ok().finish()
}
