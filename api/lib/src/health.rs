use actix_web::{ web::{ self, ServiceConfig }, HttpResponse };
use sqlx::PgPool;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn hello_world() -> &'static str {
    "Hello World!"
}

#[tracing::instrument]
async fn version(db: web::Data<PgPool>) -> String {
    tracing::info!("Getting version");

    let result: Result<String, sqlx::Error> = sqlx
        ::query_scalar("SELECT version()")
        .fetch_one(db.get_ref()).await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Failed to fetch database version: {:?}", e),
    }
}

const API_VERSION: &str = "0.0.1";

#[tracing::instrument]
async fn health() -> HttpResponse {
    HttpResponse::Ok().append_header(("version", API_VERSION)).finish()
}

#[cfg(test)]
mod test {
    use actix_web::{ http::StatusCode, App };

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let response = health().await;

        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);

        let data = response
            .headers()
            .get("version")
            .and_then(|v| v.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }

    #[actix_rt::test]
    async fn health_check_works_endpoint() {
        let app = App::new().configure(service);

        let mut app = actix_web::test::init_service(app).await;

        let request = actix_web::test::TestRequest::get().uri("/health").to_request();

        let response = actix_web::test::call_service(&mut app, request).await;

        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);

        let data = response
            .headers()
            .get("version")
            .and_then(|v| v.to_str().ok());

        assert_eq!(data, Some(API_VERSION))
    }
}
