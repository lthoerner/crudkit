use sqlx::{query_builder::QueryBuilder, Postgres};

pub const SQL_PARAMETER_BIND_LIMIT: usize = u16::MAX as usize;

#[derive(Clone)]
pub struct Database {
    pub connection: sqlx::PgPool,
}

impl Database {
    pub async fn execute_query_builder<'a>(&self, mut query_builder: QueryBuilder<'a, Postgres>) {
        query_builder
            .build()
            .execute(&self.connection)
            .await
            .unwrap();
    }
}
