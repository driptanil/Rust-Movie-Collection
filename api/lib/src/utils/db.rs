use sea_orm::{ConnectionTrait, DatabaseConnection};

pub async fn check_db_connection(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    // Perform a simple query to test the connection
    db.execute(
        sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT 1".to_string())
    ).await?;
    Ok(())
}