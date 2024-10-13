pub struct PostgresRepository {
    pub pool: sqlx::PgPool,
}

impl PostgresRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}
