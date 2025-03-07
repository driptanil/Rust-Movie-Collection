use sea_orm::{
    prelude::Uuid,
    ActiveModelBehavior,
    ActiveModelTrait,
    DatabaseConnection,
    DbErr,
    EntityTrait,
    IntoActiveModel,
    ModelTrait,
    PrimaryKeyTrait,
};

type RepoResult<T> = Result<T, DbErr>;

#[async_trait::async_trait]
pub trait BaseRepository<Model, Entity, ActiveModel>
    : Send + Sync + 'static
    where
        Model: ModelTrait + IntoActiveModel<ActiveModel> + Send + Sync,
        Entity: EntityTrait<Model = Model> + Send + Sync,
        ActiveModel: ActiveModelTrait<Entity = Entity> + ActiveModelBehavior + Send + Sync + 'static
{
    async fn get_all(&self) -> RepoResult<Vec<Model>>;
    async fn get_by_id(&self, id: Uuid) -> RepoResult<Option<Model>>;
    async fn create(&self, entity: ActiveModel) -> RepoResult<Model>;
    async fn update(&self, entity: ActiveModel) -> RepoResult<Model>;
    async fn delete(&self, id: Uuid) -> RepoResult<Uuid>;
}

#[async_trait::async_trait]
impl<Model, Entity, ActiveModel> BaseRepository<Model, Entity, ActiveModel>
    for DatabaseConnection
    where
        Model: ModelTrait<Entity = Entity> +
            IntoActiveModel<ActiveModel> +
            Send +
            Sync +
            sea_orm::FromQueryResult,
        Entity: EntityTrait<Model = Model> + Send + Sync,
        ActiveModel: ActiveModelTrait<Entity = Entity> +
            ActiveModelBehavior +
            Send +
            Sync +
            'static,
        <<Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<Uuid> + Into<Uuid>
{
    async fn get_all(&self) -> RepoResult<Vec<Model>> {
        Entity::find().all(self).await
    }

    async fn get_by_id(&self, id: Uuid) -> RepoResult<Option<Model>> {
        Entity::find_by_id(id).one(self).await
    }

    async fn create(&self, entity: ActiveModel) -> RepoResult<Model> {
        let id: Uuid = Entity::insert(entity).exec(self).await?.last_insert_id.into();

        let inserted_entity = Entity::find_by_id(id).one(self).await?;
        Ok(inserted_entity.expect("Entity not found"))
    }

    async fn update(&self, entity: ActiveModel) -> RepoResult<Model> {
        entity.update(self).await
    }

    async fn delete(&self, id: Uuid) -> RepoResult<Uuid> {
        let to_delete = Entity::find_by_id(id).one(self).await?;
        to_delete
            .expect("Movie not found")
            .delete(self).await
            .map(|_| id.to_owned())
    }
}
