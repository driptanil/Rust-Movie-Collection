use actix_web::{ get, post, delete, web, HttpResponse };
use sea_orm::prelude::Uuid;
use shared::models::review::{ CreateReviewRequest, Review };
use crate::{
    services::review_service::{ ReviewService, ReviewServiceImpl },
    utils::error::ApiResult,
};

type Service = web::Data<ReviewServiceImpl>;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/review").service(get_all).service(post));
}

#[utoipa::path(
    context_path = "/api/reviews",
     params(("movie_id" = Uuid,)),
    responses((status = 200, description = "Get List Reviews", body = Vec<Review>)),
    tag = "Review"
)]
#[get("/{movie_id}")]
pub async fn get_all(movie_id: web::Path<Uuid>, service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Getting all reviews");

    match service.get_review_by_movie_id(*movie_id).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api/reviews",
    responses((status = 200, description = "Create Review", body = Review)),
    tag = "Review"
)]
#[post("")]
async fn post(
    request: web::Json<CreateReviewRequest>,
    service: Service
) -> ApiResult<HttpResponse> {
    tracing::info!("Creating a new review");

    match service.create_movie(request.into_inner()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}
