#[derive(Clone)]
pub struct PostgresRepository {
    pub pool: sea_orm::DatabaseConnection,
}

impl PostgresRepository {
    pub fn new(pool: sea_orm::DatabaseConnection) -> Self {
        Self { pool }
    }
}
