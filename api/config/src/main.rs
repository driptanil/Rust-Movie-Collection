use actix_web::web;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sea_orm::{ Database, DatabaseConnection };
use api_lib::{
    repositories::AppRepository,
    routers::init_routes, services::movie_service::MovieServiceImpl,
};
use sea_orm::entity::prelude::*;

mod docs;
use docs::init_docs;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: String
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // Initialize SeaORM connection
    let db = Database::connect(&pool).await.map_err(CustomError::new)?;

    check_db_connection(&db).await.map_err(CustomError::new)?;

    let repo: Box<dyn AppRepository> = Box::new(db);
    // let repo = web::Data::new(repo);

      // Initialize MovieServiceImpl with the boxed repository
    let movie_service = MovieServiceImpl::new(repo);

    // Register the service with Actix as shared data
    let movie_service_data = web::Data::new(movie_service);

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(repo.clone()).configure(init_routes).configure(init_docs);
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
