use shared::models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest };
use sqlx::types::Uuid;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, RepoError>;

#[async_trait::async_trait]
pub trait MovieRepository: Send + Sync + 'static {
    async fn get_movies(&self) -> RepoResult<Vec<Movie>>;
    // Additional methods can be defined here as async
    async fn get_movie(&self, movie_id: &Uuid) -> RepoResult<Movie>;
    async fn create_movie(&self, movie: &CreateMovieRequest) -> RepoResult<Movie>;
    async fn update_movie(&self, movie: &UpdateMovieRequest) -> RepoResult<Movie>;
    async fn delete_movie(&self, movie_id: &Uuid) -> RepoResult<Uuid>;
}
