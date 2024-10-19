use actix_web::{ get, HttpResponse, Responder };
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(api_lib::routers::version::get),
    components(schemas()),
    tags(
        (name = "Auth", description = "A authentication routes"),
        (name = "Todo", description = "A todo routes"),
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
