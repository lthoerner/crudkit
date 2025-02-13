#[derive(Clone)]
pub struct Database {
    pub connection: sqlx::PgPool,
}
