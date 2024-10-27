use sea_orm::{ prelude::Uuid, EntityTrait, ActiveModelTrait, ModelTrait, Set };
use shared::{ entities::movie, models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest } };
use crate::db::postgres::PostgresConnection;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, RepoError>;

#[async_trait::async_trait]
pub trait MovieRepository: Send + Sync + 'static {
    async fn get_movies(&self) -> RepoResult<Vec<Movie>>;
    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Movie>;
    async fn create_movie(&self, movie: CreateMovieRequest) -> RepoResult<Movie>;
    async fn bulk_create_movie(&self, movie: Vec<CreateMovieRequest>) -> RepoResult<bool>;
    async fn update_movie(&self, movie: UpdateMovieRequest) -> RepoResult<Movie>;
    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid>;
}

#[async_trait::async_trait]
impl MovieRepository for PostgresConnection {
    async fn get_movies(&self) -> RepoResult<Vec<Movie>> {
        let response = movie::Entity
            ::find()
            .all(&self.pool).await
            .map_err(|e| e.to_string())?;

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
        let response = movie::Entity
            ::find_by_id(movie_id)
            .one(&self.pool).await
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

        let inserted_movie = new_movie.insert(&self.pool).await.map_err(|e| e.to_string())?;

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

    async fn bulk_create_movie(&self, movies: Vec<CreateMovieRequest>) -> RepoResult<bool> {
        let mut active_models: Vec<movie::ActiveModel> = Vec::with_capacity(movies.len());

        for movie in movies {
            let new_movie = movie::ActiveModel {
                title: Set(movie.title),
                director: Set(movie.director),
                year: Set(movie.year as i16),
                poster: Set(movie.poster),
                ..Default::default()
            };
            active_models.push(new_movie);
        }

        movie::Entity
            ::insert_many(active_models)
            .exec_with_returning(&self.pool).await
            .map_err(|e| e.to_string())?;

        Ok(true)
    }

    async fn update_movie(&self, movie: UpdateMovieRequest) -> RepoResult<Movie> {
        let movie_to_update = movie::Entity
            ::find_by_id(movie.id)
            .one(&self.pool).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Movie not found".to_string())?;

        let mut active_model: movie::ActiveModel = movie_to_update.into();
        active_model.title = Set(movie.title);
        active_model.director = Set(movie.director);
        active_model.year = Set(movie.year as i16);
        active_model.poster = Set(movie.poster);

        let updated_movie = active_model.update(&self.pool).await.map_err(|e| e.to_string())?;

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
        let movie_to_delete = movie::Entity
            ::find_by_id(movie_id)
            .one(&self.pool).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Movie not found".to_string())?;

        movie_to_delete
            .delete(&self.pool).await
            .map(|_| movie_id.to_owned())
            .map_err(|e| e.to_string())
    }
}
