use actix_web::{ get, post, put, delete, web, HttpResponse };
use actix_web_httpauth::middleware::HttpAuthentication;
use sea_orm::prelude::Uuid;
use shared::models::movie::{ CreateMovieRequest, UpdateMovieRequest };
use crate::{
    middleware::auth_middleware::auth_middleware,
    services::movie_service::{ MovieService, MovieServiceImpl },
    utils::error::ApiResult,
};

type Service = web::Data<MovieServiceImpl>;

pub fn router(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth_middleware);

    cfg.service(
        web
            ::scope("/movies")
            .wrap(auth)
            .service(get_all)
            .service(get_by_id)
            .service(post)
            .service(bulk_post)
            .service(put)
            .service(bulk_put)
            .service(delete)
    );
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Get List Movies", body = Vec<Movie>)),
    tag = "Movie"
)]
#[get("")]
pub async fn get_all(service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Getting all movies");

    match service.get_movies().await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    params(("movie_id" = Uuid,)),
    responses((status = 200, description = "Get Movie By Id", body = Movie)),
    tag = "Movie"
)]
#[get("/{movie_id}")]
pub async fn get_by_id(movie_id: web::Path<Uuid>, service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Getting movie by id");

    match service.get_movie(*movie_id).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Create Movie", body = Movie)),
    tag = "Movie"
)]
#[post("")]
async fn post(request: web::Json<CreateMovieRequest>, service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Creating a new movie");

    match service.create_movie(request.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Bulk Create Movie", body = Movie)),
    tag = "Movie"
)]
#[post("/bulk")]
async fn bulk_post(
    request: web::Json<Vec<CreateMovieRequest>>,
    service: Service
) -> ApiResult<HttpResponse> {
    tracing::info!("Creating bulk movies");

    match service.bulk_create_movie(request.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Update Movie", body = Movie)),
    tag = "Movie"
)]
#[put("")]
async fn put(request: web::Json<UpdateMovieRequest>, service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Updating a movie");

    match service.update_movie(request.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    responses((status = 200, description = "Bulk Update Movie", body = Movie)),
    tag = "Movie"
)]
#[put("/bulk")]
async fn bulk_put(
    request: web::Json<Vec<UpdateMovieRequest>>,
    service: Service
) -> ApiResult<HttpResponse> {
    tracing::info!("Updating bulk movies");

    match service.bulk_update_movie(request.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api/movies",
    params(("movie_id" = Uuid,)),
    responses((status = 200, description = "Delete Movie", body = Uuid)),
    tag = "Movie"
)]
#[delete("/{movie_id}")]
async fn delete(movie_id: web::Path<Uuid>, service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Deleting a movie");

    match service.delete_movie(*movie_id).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}
