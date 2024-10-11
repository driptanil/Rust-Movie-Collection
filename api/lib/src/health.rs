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

#[tracing::instrument]
async fn health() -> HttpResponse {
    HttpResponse::Ok().append_header(("version", "0.0.1")).finish()
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;

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

        assert_eq!(data, Some("0.0.1"));
    }
}
