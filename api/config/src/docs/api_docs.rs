use actix_web::{ get, HttpResponse, Responder };
use utoipa::OpenApi;
use shared::models::{ movie, user, user_session, version };
use api_lib::routers::{ version_router, movies_router, user_router };

#[derive(OpenApi)]
#[openapi(
    paths(
        // version_router::get,
        movies_router::get_all,
        movies_router::get_by_id,
        movies_router::post,
        movies_router::bulk_post,
        movies_router::put,
        movies_router::bulk_put,
        movies_router::delete,
        user_router::get_token,
        user_router::create_user,
        user_router::refresh_token
    ),
    components(
        schemas(
            // version::Version,
            movie::Movie,
            movie::CreateMovieRequest,
            movie::UpdateMovieRequest,
            user::User,
            user::LoginRequest,
            user::CreateUserRequest,
            user_session::UserSession,
            user_session::RefreshUserSessionRequest
        )
    ),
    tags(
        (name = "Movie", description = "Movies routes"),
        (name = "Info", description = "Info routes"),
        (name = "Token", description = "Token routes")
    )
)]
pub struct ApiDoc;

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}
