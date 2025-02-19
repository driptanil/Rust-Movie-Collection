use sea_orm::{
    prelude::Uuid,
    ColumnTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    QueryFilter,
    QueryOrder,
};
use shared::entities::{ user, user_password };

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_users(&self) -> RepoResult<Vec<user::Model>>;
    async fn get_user(&self, user_id: Uuid) -> RepoResult<Option<user::Model>>;
    async fn get_user_by_email(&self, email: String) -> RepoResult<Option<user::Model>>;
    async fn create_user(&self, user: user::ActiveModel) -> RepoResult<user::Model>;
    async fn create_user_password(
        &self,
        user_password: user_password::ActiveModel
    ) -> RepoResult<user_password::Model>;
    async fn get_user_password(&self, user_id: Uuid) -> RepoResult<Option<user_password::Model>>;
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

    async fn create_user_password(
        &self,
        user_password: user_password::ActiveModel
    ) -> RepoResult<user_password::Model> {
        let id: Uuid = user_password::Entity
            ::insert(user_password)
            .exec(self).await?.last_insert_id;

        let inserted_user_password = user_password::Entity::find_by_id(id).one(self).await?;

        Ok(inserted_user_password.expect("User Password not found"))
    }

    async fn get_user_password(&self, user_id: Uuid) -> RepoResult<Option<user_password::Model>> {
        user_password::Entity
            ::find()
            .filter(user_password::Column::UserId.eq(user_id))
            .order_by_desc(user_password::Column::CreatedAt)
            .one(self).await
    }
}
