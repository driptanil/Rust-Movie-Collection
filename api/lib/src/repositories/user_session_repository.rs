use sea_orm::{
    prelude::Uuid,
    DatabaseConnection,
    QueryFilter,
    ColumnTrait,
    DbErr,
    EntityTrait,
    ModelTrait,
};
use shared::entities::{ user, user_session };

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait UserSessionRepository: Send + Sync + 'static {
    async fn get_session(
        &self,
        access_token: String,
        refresh_token: String
    ) -> RepoResult<Option<user_session::Model>>;
    async fn create_user_session(
        &self,
        user_session: user_session::ActiveModel
    ) -> RepoResult<user_session::Model>;
    async fn delete_user_session(&self, user_session_id: Uuid) -> RepoResult<Uuid>;
}

#[async_trait::async_trait]
impl UserSessionRepository for DatabaseConnection {
    async fn get_session(
        &self,
        access_token: String,
        refresh_token: String
    ) -> RepoResult<Option<user_session::Model>> {
        user_session::Entity
            ::find()
            .filter(
                user_session::Column::AccessToken
                    .eq(access_token)
                    .and(user_session::Column::RefreshToken.eq(refresh_token))
            )
            .one(self).await
    }

    async fn create_user_session(
        &self,
        user_session: user_session::ActiveModel
    ) -> RepoResult<user_session::Model> {
        let user_id = match &user_session.user_id {
            sea_orm::ActiveValue::Set(id) => *id,
            _ => {
                return Err(DbErr::Custom("User ID must be set".into()));
            }
        };

        // getting error here for the following line
        let existing_session = user_session::Entity
            ::find()
            .filter(user_session::Column::UserId.eq(user_id))
            .one(self).await?;

        // Delete if exists
        if let Some(session) = existing_session {
            session.delete(self).await?;
        }

        let id: Uuid = user_session::Entity::insert(user_session).exec(self).await?.last_insert_id;

        let inserted_user_session = user_session::Entity::find_by_id(id).one(self).await?;

        Ok(inserted_user_session.expect("User Session not found after replacement"))
    }

    async fn delete_user_session(&self, user_session_id: Uuid) -> RepoResult<Uuid> {
        let user_session_to_delete = user_session::Entity
            ::find_by_id(user_session_id)
            .one(self).await;

        match user_session_to_delete {
            Ok(user_session) => {
                return user_session
                    .expect("User Session not found")
                    .delete(self).await
                    .map(|_| user_session_id.to_owned());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
