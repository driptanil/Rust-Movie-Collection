use actix_web::{ get, HttpResponse, Responder };
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(api_lib::routers::version::get),
    components(schemas(shared::models::version::Version)),
    tags(
        (name = "Movie", description = "Movies routes"),
        (name = "Server Metadata", description = "A server metadata routes")
    )
)]
pub struct ApiDoc;

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}
