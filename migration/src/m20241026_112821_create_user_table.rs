use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                        .default(Expr::cust("uuid_generate_v4()"))
                )
                .col(ColumnDef::new(User::Name).string().not_null())
                .col(ColumnDef::new(User::Email).string().not_null())
                .col(ColumnDef::new(User::EmailVerified).timestamp_with_time_zone().null())
                .col(ColumnDef::new(User::Image).string().null())
                .col(
                    ColumnDef::new(User::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(User::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .to_owned()
        ).await?;

        let create_trigger_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TRIGGER update_user_updated_at
            BEFORE UPDATE
            ON "user"
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
            DROP TRIGGER IF EXISTS update_user_updated_at ON "user";
            "#.to_owned()
        );
        manager.get_connection().execute(drop_trigger_stmt).await?;

        // Drop table
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    EmailVerified,
    Image,
    CreatedAt,
    UpdatedAt,
}
