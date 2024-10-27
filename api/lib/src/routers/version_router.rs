use actix_web::{ get, web, HttpResponse };
use shared::models::version::Version;
use crate::repositories::AppRepository;
use serde_json::json;

type Repository = web::Data<Box<dyn AppRepository>>;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/version").service(get));
}

#[utoipa::path(
    context_path = "/api/version",
    responses((
        status = 200,
        description = "Get database version",
        body = Version,
        example = json!(Version {
            db: "Postgres 13.3".to_string(),
        }),
    )),
    tag = "Info"
)]
#[get("")]
pub async fn get(repo: Repository) -> HttpResponse {
    tracing::info!("Getting version");

    match repo.get_version().await {
        Ok(data) => {
            // Serialize the Version struct into JSON
            match serde_json::to_string(&data) {
                Ok(json_response) =>
                    HttpResponse::Ok().content_type("application/json").body(json_response),
                Err(_) => HttpResponse::InternalServerError().body("Failed to serialize response"),
            }
        }
        Err(e) => {
            // Create a JSON error response
            let error_response = json!({ "error": e });
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(
                    serde_json
                        ::to_string(&error_response)
                        .unwrap_or_else(|_| "{\"error\": \"Internal Server Error\"}".to_string())
                )
        }
    }
}
