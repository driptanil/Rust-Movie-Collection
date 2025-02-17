pub use sea_orm_migration::prelude::*;

mod m20240218_000001_enable_uuid;
mod m20241026_123354_create_update_at_function;
mod m20241020_174338_create_movie_table;
mod m20241026_112821_create_user_table;
mod m20241026_123923_create_user_password_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240218_000001_enable_uuid::Migration), 
            Box::new(m20241026_123354_create_update_at_function::Migration),
            Box::new(m20241020_174338_create_movie_table::Migration),
            Box::new(m20241026_112821_create_user_table::Migration),
            Box::new(m20241026_123923_create_user_password_table::Migration),
        ]
    }
}
