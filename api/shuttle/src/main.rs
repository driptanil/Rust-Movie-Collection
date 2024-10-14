use actix_web::web::{ self, ServiceConfig };
use api_lib::services::{ health, movies, version };
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool
        .execute(include_str!("../../migrations/postgres/schema.sql")).await
        .map_err(CustomError::new)?;

    check_db_connection(&pool).await.map_err(CustomError::new)?;

    let repo = api_lib::db::postgres::PostgresRepository::new(pool);
    let repo = web::Data::new(repo);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web
                ::scope("/api")
                .app_data(repo)
                .configure(health::service)
                .configure(version::service::<api_lib::db::postgres::PostgresRepository>)
                .configure(movies::service::<api_lib::db::postgres::PostgresRepository>)
        );
    };

    Ok(config.into())
}

async fn check_db_connection(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // Perform a simple query to test connection
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
