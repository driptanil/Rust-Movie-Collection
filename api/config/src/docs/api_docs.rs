use actix_web::{ get, HttpResponse, Responder };
use utoipa::OpenApi;
use shared::models;
use api_lib::routers;

#[derive(OpenApi)]
#[openapi(
    paths(
        routers::version::get,
        routers::movies::get_all,
        routers::movies::get_by_id,
        routers::movies::post,
        routers::movies::bulk_post,
        routers::movies::put,
        routers::movies::delete
    ),
    components(
        schemas(
            models::version::Version,
            models::movie::Movie,
            models::movie::CreateMovieRequest,
            models::movie::UpdateMovieRequest
        )
    ),
    tags(
        (name = "Movie", description = "Movies routes"),
        (name = "Info", description = "Info routes")
    )
)]
pub struct ApiDoc;

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}
