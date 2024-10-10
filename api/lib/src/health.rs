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
