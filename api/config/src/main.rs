use actix_web::web;
use drip_rust_movie_collection::docs::init_docs;
use migration::{ Migrator, MigratorTrait };
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sea_orm::Database;
use api_lib::{
    repositories::{
        movie_repository::MovieRepository,
        user_password_repository::UserPasswordRepository,
        user_repository::UserRepository,
        user_session_repository::UserSessionRepository,
    },
    routers::init_routes,
    services::{ movie_service::MovieServiceImpl, user_service::UserServiceImpl },
    utils::db::check_db_connection,
};
use std::sync::Arc;
use actix_cors::Cors;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: String
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // Initialize SeaORM connection
    let db = Database::connect(&pool).await.map_err(CustomError::new)?;

    Migrator::up(&db, None).await.map_err(CustomError::new)?;

    check_db_connection(&db).await.map_err(CustomError::new)?;

    // Repositories
    let movie_repository: Arc<dyn MovieRepository> = Arc::new(db.clone()) as Arc<
        dyn MovieRepository
    >;
    let user_repository = Arc::new(db.clone()) as Arc<dyn UserRepository>;
    let user_session_repository = Arc::new(db.clone()) as Arc<dyn UserSessionRepository>;
    let user_password_repository = Arc::new(db.clone()) as Arc<dyn UserPasswordRepository>;

    // Services
    let movie_service = web::Data::new(MovieServiceImpl::new(movie_repository.clone()));
    let user_service = web::Data::new(
        UserServiceImpl::new(
            user_repository.clone(),
            user_session_repository.clone(),
            user_password_repository.clone()
        )
    );

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(user_service.clone())
            .app_data(movie_service.clone())
            .configure(init_docs)
            .configure(init_routes);
    };

    Ok(config.into())
}
