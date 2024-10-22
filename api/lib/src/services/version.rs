use sea_orm::{ ConnectionTrait, Statement };
use shared::models::version::Version;

use crate::{
    db::postgres::PostgresRepository,
    repositories::version::{ VersionRepository, VersionResult },
};

#[async_trait::async_trait]
impl VersionRepository for PostgresRepository {
    async fn get_version(&self) -> VersionResult<Version> {
        let raw_sql = Statement::from_string(
            self.pool.get_database_backend(),
            "SELECT version()".to_string()
        );

        // Execute the query and fetch the result as a string
        let result = self.pool.query_one(raw_sql).await;

        match result {
            Ok(Some(row)) => {
                let version: String = row.try_get("", "version").map_err(|e| e.to_string())?;
                Ok(Version { db: version })
            }
            Ok(None) => Err("No version found".to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
