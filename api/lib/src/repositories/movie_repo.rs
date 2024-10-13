use shared::models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest };
use sqlx::types::Uuid;

pub type MovieError = String;
pub type MovieResult<T> = Result<T, MovieError>;

#[async_trait::async_trait]
pub trait MovieRepository: Send + Sync + 'static {
    async fn get_movies(&self) -> MovieResult<Vec<Movie>>;
    // Additional methods can be defined here as async
    async fn get_movie(&self, movie_id: &Uuid) -> MovieResult<Movie>;
    async fn create_movie(&self, movie: &CreateMovieRequest) -> MovieResult<Movie>;
    async fn update_movie(&self, movie: &UpdateMovieRequest) -> MovieResult<Movie>;
    async fn delete_movie(&self, movie_id: &Uuid) -> MovieResult<Uuid>;
}
