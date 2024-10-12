use actix_web::web;
use sqlx::PgPool;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.route("/version", web::get().to(version));
}

#[tracing::instrument]
pub async fn version(db: web::Data<PgPool>) -> String {
    tracing::info!("Getting version");

    let result: Result<String, sqlx::Error> = sqlx
        ::query_scalar("SELECT version()")
        .fetch_one(db.get_ref()).await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Failed to fetch database version: {:?}", e),
    }
}
