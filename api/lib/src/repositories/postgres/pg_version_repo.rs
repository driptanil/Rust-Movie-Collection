use crate::{
    db::postgres::PostgresRepository,
    repositories::version_repo::{ VersionRepository, VersionResult },
};

#[async_trait::async_trait]
impl VersionRepository for PostgresRepository {
    async fn get_version(&self) -> VersionResult<String> {
        sqlx::query_scalar("SELECT version()")
            .fetch_one(&self.pool).await
            .map_err(|e| e.to_string())
    }
}
