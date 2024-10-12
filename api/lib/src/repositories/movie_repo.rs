use shared::models::movie::{ CreateMovieRequest, Movie };
use sqlx::types::Uuid;

pub type MovieError = String;
pub type MovieResult<T> = Result<T, MovieError>;

#[async_trait::async_trait]
pub trait MovieRepo {
    fn get_movie(&self, id: &Uuid) -> MovieResult<Movie>;
    fn get_movies(&self) -> MovieResult<Vec<Movie>>;
    fn create_movie(&self, movie: &CreateMovieRequest) -> MovieResult<Movie>;
    fn update_movie(&self, id: &Uuid, movie: &Movie) -> MovieResult<Movie>;
    fn delete_movie(&self, id: &Uuid) -> MovieResult<()>;
}
