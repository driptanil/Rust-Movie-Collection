use sea_orm_migration::{ prelude::*, schema::* };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserSession::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserSession::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                        .default(Expr::cust("uuid_generate_v4()"))
                )
                .col(string(UserSession::AccessToken).not_null())
                .col(string(UserSession::RefreshToken).not_null())
                .col(ColumnDef::new(UserSession::ExpiresAt).timestamp_with_time_zone().not_null())
                .col(
                    ColumnDef::new(UserSession::RefreshExpiresAt)
                        .timestamp_with_time_zone()
                        .not_null()
                )
                .col(
                    ColumnDef::new(UserSession::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(UserSession::UserId).uuid().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_user_session_user_id")
                        .from(UserSession::Table, UserSession::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserSession::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum UserSession {
    Table,
    Id,
    AccessToken,
    RefreshToken,
    CreatedAt,
    ExpiresAt,
    RefreshExpiresAt,

    UserId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
