use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, create the table
        manager.create_table(
            Table::create()
                .table(Movie::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Movie::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                        .default(Expr::cust("uuid_generate_v4()"))
                )
                .col(ColumnDef::new(Movie::Title).string().not_null())
                .col(ColumnDef::new(Movie::Director).string().not_null())
                .col(ColumnDef::new(Movie::Year).small_integer().not_null())
                .col(ColumnDef::new(Movie::Poster).string().null())
                .col(
                    ColumnDef::new(Movie::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(Movie::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .to_owned()
        ).await?;

        // Create the function separately
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

        // Create the trigger separately
        let create_trigger_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TRIGGER update_movie_updated_at
            BEFORE UPDATE
            ON movie
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column();
            "#.to_owned()
        );

        manager.get_connection().execute(create_trigger_stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop trigger separately
        let drop_trigger_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            DROP TRIGGER IF EXISTS update_movie_updated_at ON movie;
            "#.to_owned()
        );
        manager.get_connection().execute(drop_trigger_stmt).await?;

        // Drop function separately
        let drop_function_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            DROP FUNCTION IF EXISTS update_updated_at_column;
            "#.to_owned()
        );
        manager.get_connection().execute(drop_function_stmt).await?;

        // Drop table
        manager.drop_table(Table::drop().table(Movie::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Movie {
    Table,
    Id,
    Title,
    Director,
    Year,
    Poster,
    CreatedAt,
    UpdatedAt,
}
