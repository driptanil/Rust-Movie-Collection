use actix_web::web;
use drip_rust_movie_collection::docs::init_docs;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sea_orm::{ Database, DatabaseConnection };
use api_lib::{
    repositories::movie_repository::MovieRepository,
    routers::init_routes, services::movie_service::MovieServiceImpl,
};
use sea_orm::entity::prelude::*;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: String
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // Initialize SeaORM connection
    let db = Database::connect(&pool).await.map_err(CustomError::new)?;

    check_db_connection(&db).await.map_err(CustomError::new)?;

    // Wrap DatabaseConnection in Arc and pass to MovieServiceImpl
    let movie_repository = Arc::new(db) as Arc<dyn MovieRepository>;
    let movie_service = web::Data::new(MovieServiceImpl::new(movie_repository.clone()));

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(movie_service.clone())
           .configure(init_routes)
           .configure(init_docs);
    };

    Ok(config.into())
}

async fn check_db_connection(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    // Perform a simple query to test the connection
    db.execute(
        sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT 1".to_string())
    ).await?;
    Ok(())
}
