use shared::{ entities::movie, models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest } };
use sea_orm::{ prelude::Uuid, Set };
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

        let movies: Vec<Movie> = response
            .into_iter()
            .map(|movie| Movie {
                id: movie.id,
                title: movie.title,
                director: movie.director,
                year: movie.year as i16,
                poster: movie.poster,
                created_at: movie.created_at,
                updated_at: movie.updated_at,
            })
            .collect();

        Ok(movies)
    }

    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Movie> {
        let response = Repo::get_movie(self, movie_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Movie not found".to_string())?;

        Ok(Movie {
            id: response.id,
            title: response.title,
            director: response.director,
            year: response.year as i16,
            poster: response.poster,
            created_at: response.created_at,
            updated_at: response.updated_at,
        })
    }

    async fn create_movie(&self, movie: CreateMovieRequest) -> RepoResult<Movie> {
        let new_movie = movie::ActiveModel {
            title: Set(movie.title),
            director: Set(movie.director),
            year: Set(movie.year as i16),
            poster: Set(movie.poster),
            ..Default::default()
        };

        let inserted_movie = Repo::create_movie(self, new_movie).await.map_err(|e| e.to_string())?;

        Ok(Movie {
            id: inserted_movie.id.clone(),
            title: inserted_movie.title.clone(),
            director: inserted_movie.director.clone(),
            year: inserted_movie.year.clone() as i16,
            poster: inserted_movie.poster.clone(),
            created_at: inserted_movie.created_at,
            updated_at: inserted_movie.updated_at,
        })
    }

    // async fn bulk_create_movie(&self, movies: Vec<CreateMovieRequest>) -> RepoResult<bool> {
    //     Repo::bulk_create_movie(self, movies).await
    // }

    async fn update_movie(&self, movie: UpdateMovieRequest) -> RepoResult<Movie> {
        let request = movie::ActiveModel {
            title: Set(movie.title),
            director: Set(movie.director),
            year: Set(movie.year as i16),
            poster: Set(movie.poster),
            ..Default::default()
        };

        let updated_movie = Repo::update_movie(self, request).await.map_err(|e| e.to_string())?;

        Ok(Movie {
            id: updated_movie.id.clone(),
            title: updated_movie.title.clone(),
            director: updated_movie.director.clone(),
            year: updated_movie.year.clone() as i16,
            poster: updated_movie.poster.clone(),
            created_at: updated_movie.created_at,
            updated_at: updated_movie.updated_at,
        })
    }

    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid> {
        Repo::delete_movie(self, movie_id).await.map_err(|e| e.to_string());

        Ok(movie_id)
    }
}
