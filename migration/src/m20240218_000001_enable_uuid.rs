use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Execute raw SQL to create the extension
        let db = manager.get_connection();
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";").await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the extension if needed
        let db = manager.get_connection();

        db.execute_unprepared("DROP EXTENSION IF EXISTS \"uuid-ossp\";").await?;

        Ok(())
    }
}