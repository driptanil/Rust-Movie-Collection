use sea_orm::{
    prelude::Uuid,
    ActiveModelBehavior,
    ActiveModelTrait,
    DbErr,
    EntityTrait,
    IntoActiveModel,
    ModelTrait,
    PrimaryKeyTrait,
};
use crate::db::postgres::PostgresConnection;

type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait BaseRepository<Model, ActiveModel>
    : Send + Sync + 'static
    where
        Model: EntityTrait + ModelTrait + Send + Sync,
        ActiveModel: ActiveModelTrait<Entity = Model> + ActiveModelBehavior + Send + Sync,
        <<Model as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<Uuid>
{
    async fn get_all(&self) -> RepoResult<Vec<Model::Model>>;
    async fn get_by_id(&self, id: Uuid) -> RepoResult<Option<Model::Model>>;
    async fn create(&self, entity: ActiveModel) -> RepoResult<Model::Model>;
    async fn update(&self, entity: ActiveModel) -> RepoResult<Model::Model>;
    async fn delete(&self, id: Uuid) -> RepoResult<()>;
}

#[async_trait::async_trait]
impl<Model, ActiveModel> BaseRepository<Model, ActiveModel>
    for PostgresConnection
    where
        Model: EntityTrait + ModelTrait + Send + Sync,
        ActiveModel: ActiveModelTrait<Entity = Model> + ActiveModelBehavior + Send + Sync + 'static,
        <<Model as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<Uuid>,
        <Model as EntityTrait>::Model: IntoActiveModel<ActiveModel>
{
    async fn get_all(&self) -> RepoResult<Vec<Model::Model>> {
        let response = Model::find().all(&self.pool).await?;
        Ok(response)
    }

    async fn get_by_id(&self, id: Uuid) -> RepoResult<Option<Model::Model>> {
        let primary_key_value =
            <<Model as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType::from(id);
        let response = Model::find_by_id(primary_key_value).one(&self.pool).await?;
        Ok(response)
    }

    async fn create(&self, entity: ActiveModel) -> RepoResult<Model::Model> {
        let result = entity.insert(&self.pool).await?;
        Ok(result)
    }

    async fn update(&self, entity: ActiveModel) -> RepoResult<Model::Model> {
        let updated = entity.update(&self.pool).await?;
        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> RepoResult<()> {
        let primary_key_value =
            <<Model as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType::from(id);
        let entity = Model::find_by_id(primary_key_value)
            .one(&self.pool).await?
            .ok_or(DbErr::RecordNotFound("Entity not found".to_string()))?;

        entity.delete(&self.pool).await?;
        Ok(())
    }
}
