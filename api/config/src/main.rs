use actix_web::web;
use drip_rust_movie_collection::docs::init_docs;
use migration::{Migrator, MigratorTrait};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sea_orm::Database ;
use api_lib::{
    repositories::{movie_repository::MovieRepository, user_repository::UserRepository},
    routers::init_routes, services::{movie_service::MovieServiceImpl, user_service::UserServiceImpl}, utils::db::check_db_connection,
};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: String
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // Initialize SeaORM connection
    let db = Database::connect(&pool).await.map_err(CustomError::new)?;

    Migrator::up(&db, None).await.map_err(CustomError::new)?;

    check_db_connection(&db).await.map_err(CustomError::new)?;

    // Wrap DatabaseConnection in Arc and pass to MovieServiceImpl
    let movie_repository = Arc::new(db.clone()) as Arc<dyn MovieRepository>;
    let movie_service = web::Data::new(MovieServiceImpl::new(movie_repository.clone()));

    let user_repository = Arc::new(db.clone()) as Arc<dyn UserRepository>;
    let user_service = web::Data::new(UserServiceImpl::new(user_repository.clone()));

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(user_service.clone())
            .app_data(movie_service.clone())
           .configure(init_routes)
           .configure(init_docs);
    };

    Ok(config.into())
}
