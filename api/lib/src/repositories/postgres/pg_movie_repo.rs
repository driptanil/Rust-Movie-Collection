use shared::models::movie::{ CreateMovieRequest, Movie };
use sqlx::types::Uuid;

use crate::{
    db::postgres::PostgresRepository,
    repositories::movie_repo::{ MovieRepository, MovieResult },
};

#[async_trait::async_trait]
impl MovieRepository for PostgresRepository {
    async fn get_movies(&self) -> MovieResult<Vec<Movie>> {
        sqlx::query_as::<_, Movie>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM movies
            "#
        )
            .fetch_all(&self.pool).await
            .map_err(|e| e.to_string())
    }

    async fn get_movie(&self, movie_id: &Uuid) -> MovieResult<Movie> {
        sqlx::query_as::<_, Movie>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM movies
            WHERE id = $1
            "#
        )
            .bind(movie_id)
            .fetch_one(&self.pool).await
            .map_err(|e| e.to_string())
    }

    async fn create_movie(&self, movie: &CreateMovieRequest) -> MovieResult<Movie> {
        sqlx::query_as::<_, Movie>(
            r#"
            INSERT INTO movies(title, director, year, poster)
            FROM ($1, $2, $3, $4)
            RETURNING id, title, director, year, poster, created_at, updated_at
            "#
        )
            .bind(&movie.title)
            .bind(&movie.director)
            .bind(*&movie.year as i16)
            .bind(&movie.poster)
            .fetch_one(&self.pool).await
            .map_err(|e| e.to_string())
    }

    async fn update_movie(&self, movie: &Movie) -> MovieResult<Movie> {
        // Implementation for updating a movie
        sqlx::query_as::<_, Movie>(
            r#"
            UPDATE movies
            SET title = $1, director = $2, year = $3, poster = $4
            WHERE id = $5
            RETURNING id, title, director, year, poster, created_at, updated_at
            "#
        )
            .bind(&movie.title)
            .bind(&movie.director)
            .bind(*&movie.year as i16)
            .bind(&movie.poster)
            .bind(&movie.id)
            .fetch_one(&self.pool).await
            .map_err(|e| e.to_string())
    }

    async fn delete_movie(&self, movie_id: &Uuid) -> MovieResult<Uuid> {
        // Implementation for deleting a movie
        sqlx::query_scalar::<_, Uuid>(
            r#"
            DELETE FROM movies
            WHERE id = $1
            RETURNING id
            "#
        )
            .bind(movie_id)
            .fetch_one(&self.pool).await
            .map_err(|e| e.to_string())
    }
}
