use shared::models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest };
use sea_orm::prelude::Uuid;
use crate::{
    db::postgres::PostgresConnection,
    repositories::movie_repository::{ MovieRepository as Repo, RepoResult },
};

#[async_trait::async_trait]
pub trait MovieService: Send + Sync + 'static {
    async fn get_movies(&self) -> RepoResult<Vec<Movie>>;
    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Movie>;
    async fn create_movie(&self, movie: CreateMovieRequest) -> RepoResult<Movie>;
    // async fn bulk_create_movie(&self, movie: Vec<CreateMovieRequest>) -> RepoResult<bool>;
    async fn update_movie(&self, movie: UpdateMovieRequest) -> RepoResult<Movie>;
    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid>;
}

#[async_trait::async_trait]
impl MovieService for PostgresConnection {
    async fn get_movies(&self) -> RepoResult<Vec<Movie>> {
        let response = Repo::get_movies(self).await.map_err(|e| e.to_string())?;

        Ok(Movie::from_list_models(response))
    }

    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Movie> {
        let response = Repo::get_movie(self, movie_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Movie not found".to_string())?;

        Ok(Movie::from_model(response))
    }

    async fn create_movie(&self, movie: CreateMovieRequest) -> RepoResult<Movie> {
        let new_movie = movie.to_active_model();

        let inserted_movie = Repo::create_movie(self, new_movie).await.map_err(|e| e.to_string())?;

        Ok(Movie::from_model(inserted_movie))
    }

    // async fn bulk_create_movie(&self, movies: Vec<CreateMovieRequest>) -> RepoResult<bool> {
    //     Repo::bulk_create_movie(self, movies).await
    // }

    async fn update_movie(&self, movie: UpdateMovieRequest) -> RepoResult<Movie> {
        let request = movie.to_active_model();

        let updated_movie = Repo::update_movie(self, request).await.map_err(|e| e.to_string())?;

        Ok(Movie::from_model(updated_movie))
    }

    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid> {
        Repo::delete_movie(self, movie_id).await.map_err(|e| e.to_string());

        Ok(movie_id)
    }
}
