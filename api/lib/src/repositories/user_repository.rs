use sea_orm::{prelude::Uuid, DatabaseConnection, QueryFilter, ColumnTrait, DbErr, EntityTrait};
use shared::entities::user;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_users(&self) -> RepoResult<Vec<user::Model>>;
    async fn get_user(&self, user_id: Uuid) -> RepoResult<Option<user::Model>>;
    async fn get_user_by_email(&self, email: String) -> RepoResult<Option<user::Model>>;
    async fn create_user(&self, user: user::ActiveModel) -> RepoResult<user::Model>;
}

#[async_trait::async_trait]
impl UserRepository for DatabaseConnection {
    async fn get_users(&self) -> RepoResult<Vec<user::Model>> {
        user::Entity::find().all(self).await
    }

    async fn get_user(&self, user_id: Uuid) -> RepoResult<Option<user::Model>> {
        user::Entity::find_by_id(user_id).one(self).await
    }

    async fn get_user_by_email(&self, email: String) -> RepoResult<Option<user::Model>> {
        user::Entity::find().filter(user::Column::Email.eq(email)).one(self).await
    }

    async fn create_user(&self, user: user::ActiveModel) -> RepoResult<user::Model> {
        let id: Uuid = user::Entity::insert(user).exec(self).await?.last_insert_id;

        let inserted_user = user::Entity::find_by_id(id).one(self).await?;

        Ok(inserted_user.expect("User not found"))
    }
}