use actix_web::{ web, HttpResponse };
use utoipa_swagger_ui::SwaggerUi;

pub mod api_docs;

pub fn init_docs(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // OpenAPI document
        web
            ::scope("/docs")
            .service(api_docs::openapi_json)
            .route("/swagger", web::get().to(redirect_swagger))
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", Default::default())
            )
    );
}

async fn redirect_swagger() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/docs/swagger/")) // Use append_header
        .finish()
}
