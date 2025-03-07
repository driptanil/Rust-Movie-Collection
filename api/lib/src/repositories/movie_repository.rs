use sea_orm::{ ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait };
use shared::entities::movie;

use super::base_repository::BaseRepository;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait MovieRepository: BaseRepository<movie::Model, movie::Entity, movie::ActiveModel> {
    async fn bulk_create(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool>;
    async fn bulk_update(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool>;
}

#[async_trait::async_trait]
impl MovieRepository for DatabaseConnection {
    async fn bulk_create(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool> {
        movie::Entity::insert_many(movies).exec(self).await?;

        Ok(true)
    }

    async fn bulk_update(&self, movies: Vec<movie::ActiveModel>) -> RepoResult<bool> {
        for movie in movies {
            movie.update(self).await?;
        }

        Ok(true)
    }
}
