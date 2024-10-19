use actix_web::web;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use api_lib::{
    db::postgres::PostgresRepository,
    repositories::AppRepository,
    routers::init_routes,
};

mod docs;
use docs::init_docs;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    pool
        .execute(include_str!("../../migrations/postgres/schema.sql")).await
        .map_err(CustomError::new)?;

    check_db_connection(&pool).await.map_err(CustomError::new)?;

    let repo: Box<dyn AppRepository> = Box::new(PostgresRepository::new(pool));
    let repo = web::Data::new(repo);

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(repo.clone()).configure(init_routes).configure(init_docs);
    };

    Ok(config.into())
}

async fn check_db_connection(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // Perform a simple query to test connection
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
