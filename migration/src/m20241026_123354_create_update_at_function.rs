use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_function_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = CURRENT_TIMESTAMP;
                RETURN NEW;
            END;
            $$ language 'plpgsql';
            "#.to_owned()
        );

        manager.get_connection().execute(create_function_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop function separately
        let drop_function_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            DROP FUNCTION IF EXISTS update_updated_at_column;
            "#.to_owned()
        );
        manager.get_connection().execute(drop_function_stmt).await?;

        Ok(())
    }
}
