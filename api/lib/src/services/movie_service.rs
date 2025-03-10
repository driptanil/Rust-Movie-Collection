use shared::{ entities::movie, models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest } };
use sea_orm::prelude::Uuid;
use std::sync::Arc;
use crate::{
    repositories::movie_repository::MovieRepository,
    utils::error::{ ApiError, ApiResult },
};

#[async_trait::async_trait]
pub trait MovieService: Send + Sync + 'static {
    async fn get_movies(&self) -> ApiResult<Vec<Movie>>;
    async fn get_movie(&self, movie_id: Uuid) -> ApiResult<Movie>;
    async fn create_movie(&self, movie: CreateMovieRequest) -> ApiResult<Movie>;
    async fn bulk_create_movie(&self, movie: Vec<CreateMovieRequest>) -> ApiResult<bool>;
    async fn update_movie(&self, movie: UpdateMovieRequest) -> ApiResult<Movie>;
    async fn bulk_update_movie(&self, movie: Vec<UpdateMovieRequest>) -> ApiResult<bool>;
    async fn delete_movie(&self, movie_id: Uuid) -> ApiResult<Uuid>;
}

pub struct MovieServiceImpl {
    repo: Arc<dyn MovieRepository>,
}

impl MovieServiceImpl {
    pub fn new(repo: Arc<dyn MovieRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl MovieService for MovieServiceImpl {
    async fn get_movies(&self) -> ApiResult<Vec<Movie>> {
        let response = self.repo
            .get_all().await
            .map_err(|e| { ApiError::InternalServer(format!("Database error: {:?}", e)) })?;
        Ok(Movie::from_list_models(response))
    }

    async fn get_movie(&self, movie_id: Uuid) -> ApiResult<Movie> {
        let response = self.repo
            .get_by_id(movie_id).await
            .map_err(|_e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        // Use `ok_or_else` to convert None into an error if the movie is not found
        let movie_model = response.ok_or_else(|| ApiError::NotFound("Movie not found".to_owned()))?;

        Ok(Movie::from_model(movie_model))
    }

    async fn create_movie(&self, movie: CreateMovieRequest) -> ApiResult<Movie> {
        let new_movie = movie.to_active_model();

        let inserted_movie = self.repo
            .create(new_movie).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(Movie::from_model(inserted_movie))
    }

    async fn bulk_create_movie(&self, movies: Vec<CreateMovieRequest>) -> ApiResult<bool> {
        let movies: Vec<movie::ActiveModel> = movies
            .into_iter()
            .map(|m| m.to_active_model())
            .collect();

        let inserted_movies = self.repo
            .bulk_create(movies).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(inserted_movies)
    }

    async fn update_movie(&self, movie: UpdateMovieRequest) -> ApiResult<Movie> {
        let request = movie.to_active_model();

        let updated_movie = self.repo
            .update(request).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(Movie::from_model(updated_movie))
    }

    async fn bulk_update_movie(&self, movies: Vec<UpdateMovieRequest>) -> ApiResult<bool> {
        let movies: Vec<movie::ActiveModel> = movies
            .into_iter()
            .map(|m| m.to_active_model())
            .collect();

        let updated_movies = self.repo
            .bulk_update(movies).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(updated_movies)
    }

    async fn delete_movie(&self, movie_id: Uuid) -> ApiResult<Uuid> {
        self.repo
            .delete(movie_id).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(movie_id)
    }
}
