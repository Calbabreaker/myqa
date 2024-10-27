#[derive(Clone)]
pub struct Store {
    pool: sqlx::postgres::PgPool,
}

impl Store {
    pub fn new(pool: sqlx::postgres::PgPool) -> Self {
        Self { pool }
    }

    pub fn create_user(&self) {}
}
