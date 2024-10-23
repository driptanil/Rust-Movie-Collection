use actix_web::{ get, post, put, delete, web, HttpResponse };
use sea_orm::prelude::Uuid;
use shared::models::movie::{ CreateMovieRequest, UpdateMovieRequest };

use crate::repositories::AppRepository;

type Repository = web::Data<Box<dyn AppRepository>>;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/movies")
            .service(get_all)
            .service(get_by_id)
            .service(post)
            .service(put)
            .service(delete)
    );
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Get List Movies", body = Vec<Movie>)),
    tag = "Movie"
)]
#[get("")]
pub async fn get_all(repo: Repository) -> HttpResponse {
    tracing::info!("Getting all movies");

    match repo.get_movies().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    params(("movie_id" = Uuid,)),
    responses((status = 200, description = "Get Movie By Id", body = Movie)),
    tag = "Movie"
)]
#[get("/{movie_id}")]
pub async fn get_by_id(movie_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    tracing::info!("Getting movie by id");

    match repo.get_movie(*movie_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Create Movie", body = Movie)),
    tag = "Movie"
)]
#[post("")]
async fn post(request: web::Json<CreateMovieRequest>, repo: Repository) -> HttpResponse {
    tracing::info!("Creating a new movie");

    match repo.create_movie(request.clone()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            tracing::error!("Database error occurred: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to create movie: {}", e))
        }
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Update Movie", body = Movie)),
    tag = "Movie"
)]
#[put("")]
async fn put(request: web::Json<UpdateMovieRequest>, repo: Repository) -> HttpResponse {
    tracing::info!("Updating a movie");

    match repo.update_movie(request.clone()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    params(("movie_id" = Uuid,)),
    responses((status = 200, description = "Delete Movie", body = Uuid)),
    tag = "Movie"
)]
#[delete("/{movie_id}")]
async fn delete(movie_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    tracing::info!("Deleting a movie");

    match repo.delete_movie(*movie_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
