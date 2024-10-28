use sea_orm::{ prelude::Uuid, ActiveModelTrait, DbErr, EntityTrait, ModelTrait };
use shared::entities::movie;
use crate::db::postgres::PostgresConnection;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait MovieRepository: Send + Sync + 'static {
    async fn get_movies(&self) -> RepoResult<Vec<movie::Model>>;
    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Option<movie::Model>>;
    async fn create_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model>;
    // async fn bulk_create_movie(&self, movie: Vec<CreateMovieRequest>) -> RepoResult<bool>;
    async fn update_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model>;
    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid>;
}

#[async_trait::async_trait]
impl MovieRepository for PostgresConnection {
    async fn get_movies(&self) -> RepoResult<Vec<movie::Model>> {
        movie::Entity::find().all(&self.pool).await
    }

    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Option<movie::Model>> {
        movie::Entity::find_by_id(movie_id).one(&self.pool).await
    }

    async fn create_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model> {
        movie.insert(&self.pool).await
    }

    // async fn bulk_create_movie(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool> {
    //     let mut active_models: Vec<movie::ActiveModel> = Vec::with_capacity(movies.len());

    //     for movie in movies {
    //         let new_movie = movie::ActiveModel {
    //             title: Set(movie.title),
    //             director: Set(movie.director),
    //             year: Set(movie.year as i16),
    //             poster: Set(movie.poster),
    //             ..Default::default()
    //         };
    //         active_models.push(new_movie);
    //     }

    //     movie::Entity
    //         ::insert_many(active_models)
    //         .exec_with_returning(&self.pool).await
    //         .map_err(|e| e.to_string())?;

    //     Ok(true)
    // }

    async fn update_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model> {
        movie.update(&self.pool).await
    }

    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid> {
        let movie_to_delete = movie::Entity::find_by_id(movie_id).one(&self.pool).await;

        match movie_to_delete {
            Ok(movie) => {
                return movie.expect("Movie not found").delete(&self.pool).await.map(|_| movie_id.to_owned());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
