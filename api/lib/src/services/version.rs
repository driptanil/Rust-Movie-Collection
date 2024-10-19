use shared::models::version::Version;

use crate::{
    db::postgres::PostgresRepository,
    repositories::version::{ VersionRepository, VersionResult },
};

#[async_trait::async_trait]
impl VersionRepository for PostgresRepository {
    async fn get_version(&self) -> VersionResult<Version> {
        let db_version: Result<String, sqlx::Error> = sqlx
            ::query_scalar("SELECT version()")
            .fetch_one(&self.pool).await;

        match db_version {
            Ok(version) => Ok(Version { db: version }),
            Err(e) => Err(e.to_string()),
        }
    }
}
