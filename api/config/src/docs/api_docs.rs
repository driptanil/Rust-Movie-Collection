use actix_web::{ get, HttpResponse, Responder };
use shared::models::{ movie, user, user_session, version };
use api_lib::routers::{ version_router, movies_router, user_router };
use utoipa::{ openapi::security::{ HttpAuthScheme, HttpBuilder, SecurityScheme }, Modify, OpenApi };

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
    ),
    modifiers(&SecurityAddon),
    security(
        ("bearer_auth" = [])
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("JWT").build()
            )
        );
        // components.add_security_scheme(
        //     "basic_auth",
        //     SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build())
        // )
    }
}

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}
