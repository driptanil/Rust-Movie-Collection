use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserPassword::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserPassword::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                        .default(Expr::cust("uuid_generate_v4()"))
                )
                .col(ColumnDef::new(UserPassword::Password).string().not_null())
                .col(ColumnDef::new(UserPassword::PasswordSalt).string().not_null())
                .col(
                    ColumnDef::new(UserPassword::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(UserPassword::UserId).uuid().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_posts_user_id")
                        .from(UserPassword::Table, UserPassword::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop table
        manager.drop_table(Table::drop().table(UserPassword::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
enum UserPassword {
    Table,
    Id,
    Password,
    PasswordSalt,
    CreatedAt,

    UserId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
