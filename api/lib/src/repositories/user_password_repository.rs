use sea_orm::{
    prelude::Uuid,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    QueryFilter,
    QueryOrder,
    ColumnTrait,
};
use shared::entities::user_password;

use super::base_repository::BaseRepository;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait UserPasswordRepository: BaseRepository<
    user_password::Model,
    user_password::Entity,
    user_password::ActiveModel
> {
    async fn get_by_user_id(&self, user_id: Uuid) -> RepoResult<Option<user_password::Model>>;
}

#[async_trait::async_trait]
impl UserPasswordRepository for DatabaseConnection {
    async fn get_by_user_id(&self, user_id: Uuid) -> RepoResult<Option<user_password::Model>> {
        user_password::Entity
            ::find()
            .filter(user_password::Column::UserId.eq(user_id))
            .order_by_desc(user_password::Column::CreatedAt)
            .one(self).await
    }
}
