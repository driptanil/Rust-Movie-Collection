use actix_web::{ web, HttpResponse };
use shared::models::movie::{ CreateMovieRequest, UpdateMovieRequest };
use sqlx::types::Uuid;

use crate::repositories::movie_repo::MovieRepository;

type Repository = web::Data<Box<dyn MovieRepository>>;

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

pub async fn get_all(repo: Repository) -> HttpResponse {
    tracing::info!("Getting all movies");

    match repo.get_movies().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_by_id(movie_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    tracing::info!("Getting movie by id");

    match repo.get_movie(&movie_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

async fn post(request: web::Json<CreateMovieRequest>, repo: Repository) -> HttpResponse {
    tracing::info!("Creating a new movie");

    match repo.create_movie(&request).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

async fn put(request: web::Json<UpdateMovieRequest>, repo: Repository) -> HttpResponse {
    tracing::info!("Updating a movie");

    match repo.update_movie(&request).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

async fn delete(movie_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    tracing::info!("Deleting a movie");

    match repo.delete_movie(&movie_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
