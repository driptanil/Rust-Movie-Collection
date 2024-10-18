use actix_web::web::{ self, ServiceConfig };
use api_lib::routers::{ health, movies, version };
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use api_lib::db::postgres::PostgresRepository;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool
        .execute(include_str!("../../migrations/postgres/schema.sql")).await
        .map_err(CustomError::new)?;

    check_db_connection(&pool).await.map_err(CustomError::new)?;

    let repo = PostgresRepository::new(pool);
    let repo = web::Data::new(repo);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web
                ::scope("/api")
                .app_data(repo)
                .configure(health::router)
                .configure(version::router::<PostgresRepository>)
                .configure(movies::router::<PostgresRepository>)
        );
    };

    Ok(config.into())
}

async fn check_db_connection(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // Perform a simple query to test connection
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
