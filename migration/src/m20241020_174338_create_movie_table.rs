use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                )
                .col(
                    ColumnDef::new(Movie::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                )
                .to_owned()
        ).await

        // ALTER TABLE public.movie ALTER COLUMN id SET DEFAULT uuid_generate_v4();
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Movie::Table).to_owned()).await
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
