use sea_orm::{ ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter };
use shared::entities::user;

use super::base_repository::BaseRepository;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait UserRepository: BaseRepository<user::Model, user::Entity, user::ActiveModel> {
    async fn get_user_by_email(&self, email: String) -> RepoResult<Option<user::Model>>;
}

#[async_trait::async_trait]
impl UserRepository for DatabaseConnection {
    async fn get_user_by_email(&self, email: String) -> RepoResult<Option<user::Model>> {
        user::Entity::find().filter(user::Column::Email.eq(email)).one(self).await
    }
}
