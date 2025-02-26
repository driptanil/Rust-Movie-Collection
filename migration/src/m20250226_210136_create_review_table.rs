use sea_orm_migration::{ prelude::*, schema::*, sea_orm::Statement };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager.create_table(
            Table::create()
                .table(Review::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Review::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                        .default(Expr::cust("uuid_generate_v4()"))
                )
                .col(ColumnDef::new(Review::Rating).decimal().not_null())
                .col(ColumnDef::new(Review::Comment).string().not_null())
                .col(
                    ColumnDef::new(Review::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(Review::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(Review::UserId).uuid().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_reviews_user_id")
                        .from(Review::Table, Review::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .col(ColumnDef::new(Review::MovieId).uuid().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_reviews_movie_id")
                        .from(Review::Table, Review::MovieId)
                        .to(Movie::Table, Movie::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await;

        let create_trigger_stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TRIGGER update_review_updated_at
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

        // Drop table
        manager.drop_table(Table::drop().table(Movie::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Review {
    Table,
    Id,
    Rating,
    Comment,
    CreatedAt,
    UpdatedAt,

    MovieId,
    UserId,
}

#[derive(Iden)]
enum Movie {
    Table,
    Id,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
