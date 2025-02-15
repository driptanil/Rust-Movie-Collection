use actix_web::{ get, post, put, delete, web, HttpResponse };
use sea_orm::prelude::Uuid;
use shared::models::user::{ CreateUserRequest, LoginRequest, UserToken };
use crate::{ services::user_service::{UserService, UserServiceImpl}, utils::error::ApiResult };

type Service = web::Data<UserServiceImpl>;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("")
            .service(get_token)
            .service(create_user)
    );
}

#[utoipa::path(
    context_path = "/api",
    responses((status = 200, description = "Get Token", body = UserToken)),
    tag = "Token"
)]
#[post("/token")]
async fn get_token(request: web::Json<LoginRequest>, service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Getting token");

    match service.get_token(request.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses((status = 200, description = "Create User", body = CreateUserRequest)),
    tag = "Token"
)]
#[post("/register")]
async fn create_user(request: web::Json<CreateUserRequest>, service: Service) -> ApiResult<HttpResponse> {
    tracing::info!("Creating User");

    match service.register(request.clone()).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    }
}