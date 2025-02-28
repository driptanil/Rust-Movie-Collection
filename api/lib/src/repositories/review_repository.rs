use sea_orm::{
    prelude::Uuid,
    ActiveModelTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    ModelTrait,
    ColumnTrait,
    QueryFilter,
};
use shared::entities::review;

pub type RepoError = String;
pub type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait ReviewRepository: Send + Sync + 'static {
    async fn get_review_by_movie_id(&self, movie_id: Uuid) -> RepoResult<Vec<review::Model>>;
    async fn create_review(&self, review: review::ActiveModel) -> RepoResult<review::Model>;
}

#[async_trait::async_trait]
impl ReviewRepository for DatabaseConnection {
    async fn get_review_by_movie_id(&self, movie_id: Uuid) -> RepoResult<Vec<review::Model>> {
        review::Entity::find().filter(review::Column::MovieId.eq(movie_id)).all(self).await
    }

    async fn create_review(&self, review: review::ActiveModel) -> RepoResult<review::Model> {
        let id: Uuid = review::Entity::insert(review).exec(self).await?.last_insert_id;

        let inserted_review = review::Entity::find_by_id(id).one(self).await?;

        Ok(inserted_review.expect("Review not found"))
    }
}
