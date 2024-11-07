use actix_web::web;
use movie_repository::MovieRepository;
use sea_orm::DatabaseConnection;

pub mod base_repository;
pub mod movie_repository;
pub mod version_repository;

pub type Repository = web::Data<Box<dyn AppRepository>>;

pub trait AppRepository: MovieRepository + Send + Sync + 'static {}

impl AppRepository for DatabaseConnection {}

// #[async_trait::async_trait]
// pub trait Repository<C, R, U, D>: Send + Sync + 'static {
//     async fn get(&self) -> RepoResult<Vec<R>>;
//     async fn get_by_id(&self, id: &Uuid) -> RepoResult<R>;
//     async fn create(&self, create: &C) -> RepoResult<R>;
//     async fn create_bulk(&self, create: &Vec<C>) -> RepoResult<Vec<R>>;
//     async fn update(&self, update: &U) -> RepoResult<R>;
//     async fn delete(&self, id: &Uuid) -> RepoResult<D>;
//     async fn delete_bulk(&self) -> RepoResult<Vec<D>>;
// }
