pub mod movie;
pub mod version;

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
