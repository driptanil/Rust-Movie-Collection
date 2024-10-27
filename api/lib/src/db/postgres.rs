#[derive(Clone)]
pub struct PostgresConnection {
    pub pool: sea_orm::DatabaseConnection,
}

impl PostgresConnection {
    pub fn new(pool: sea_orm::DatabaseConnection) -> Self {
        Self { pool }
    }
}
