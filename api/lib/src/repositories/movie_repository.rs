use sea_orm::{
    prelude::Uuid,
    ActiveModelTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    ModelTrait,
};
use shared::entities::movie;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait MovieRepository: Send + Sync + 'static {
    async fn get_movies(&self) -> RepoResult<Vec<movie::Model>>;
    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Option<movie::Model>>;
    async fn create_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model>;
    async fn bulk_create_movie(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool>;
    async fn update_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model>;
    async fn bulk_update_movie(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool>;
    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid>;
}

#[async_trait::async_trait]
impl MovieRepository for DatabaseConnection {
    async fn get_movies(&self) -> RepoResult<Vec<movie::Model>> {
        movie::Entity::find().all(self).await
    }

    async fn get_movie(&self, movie_id: Uuid) -> RepoResult<Option<movie::Model>> {
        movie::Entity::find_by_id(movie_id).one(self).await
    }

    async fn create_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model> {
        let id: Uuid = movie::Entity::insert(movie).exec(self).await?.last_insert_id;

        let inserted_movie = movie::Entity::find_by_id(id).one(self).await?;

        Ok(inserted_movie.expect("Movie not found"))
    }

    async fn bulk_create_movie(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool> {
        movie::Entity::insert_many(movies).exec(self).await?;

        Ok(true)
    }

    async fn update_movie(&self, movie: movie::ActiveModel) -> RepoResult<movie::Model> {
        movie.update(self).await
    }
    
    async fn bulk_update_movie(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool> {
        for movie in movies {
            movie.update(self).await?;
        }

        Ok(true)
    }

    async fn delete_movie(&self, movie_id: Uuid) -> RepoResult<Uuid> {
        let movie_to_delete = movie::Entity::find_by_id(movie_id).one(self).await;

        match movie_to_delete {
            Ok(movie) => {
                return movie
                    .expect("Movie not found")
                    .delete(self).await
                    .map(|_| movie_id.to_owned());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
