use actix_web::{ web, HttpResponse };
use shared::models::movie::{ CreateMovieRequest, UpdateMovieRequest };
use sqlx::types::Uuid;

use crate::repositories::movie_repo::MovieRepository as Repository;

// type Repository = web::Data<Box<dyn MovieRepository>>;

pub fn service<R: Repository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/movies")
            .route("", web::get().to(get_all::<R>))
            .route("/{movie_id}", web::get().to(get_by_id::<R>))
            .route("", web::post().to(post::<R>))
            .route("", web::put().to(put::<R>))
            .route("/{movie_id}", web::delete().to(delete::<R>))
    );
}

pub async fn get_all<R: Repository>(repo: web::Data<R>) -> HttpResponse {
    tracing::info!("Getting all movies");

    match repo.get_movies().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_by_id<R: Repository>(
    movie_id: web::Path<Uuid>,
    repo: web::Data<R>
) -> HttpResponse {
    tracing::info!("Getting movie by id");

    match repo.get_movie(&movie_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

async fn post<R: Repository>(
    request: web::Json<CreateMovieRequest>,
    repo: web::Data<R>
) -> HttpResponse {
    tracing::info!("Creating a new movie");

    match repo.create_movie(&request).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

async fn put<R: Repository>(
    request: web::Json<UpdateMovieRequest>,
    repo: web::Data<R>
) -> HttpResponse {
    tracing::info!("Updating a movie");

    match repo.update_movie(&request).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

async fn delete<R: Repository>(movie_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    tracing::info!("Deleting a movie");

    match repo.delete_movie(&movie_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
