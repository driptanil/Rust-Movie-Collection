use sea_orm::{ ConnectionTrait, Statement };

use crate::db::postgres::PostgresConnection;

pub type VersionError = String;
pub type VersionResult<T> = Result<T, VersionError>;

#[async_trait::async_trait]
pub trait VersionRepository: Send + Sync + 'static {
    async fn get_db_version(&self) -> VersionResult<String>;
}

#[async_trait::async_trait]
impl VersionRepository for PostgresConnection {
    async fn get_db_version(&self) -> VersionResult<String> {
        let raw_sql = Statement::from_string(
            self.pool.get_database_backend(),
            "SELECT version()".to_string()
        );

        // Execute the query and fetch the result as a string
        let result = self.pool.query_one(raw_sql).await;

        match result {
            Ok(Some(row)) => {
                let version: String = row.try_get("", "version").map_err(|e| e.to_string())?;
                Ok(version)
            }
            Ok(None) => Err("No version found".to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
