use sqlx::{query_builder::QueryBuilder, Postgres};

#[allow(unused_imports)]
use crate::traits::read::ReadRelation;
#[allow(unused_imports)]
use crate::traits::write::WriteRelation;

pub const SQL_PARAMETER_BIND_LIMIT: usize = u16::MAX as usize;

/// A trait that allows the usage of generic server state types.
///
/// The handler functions that are provided by [`ReadRelation`] and [`WriteRelation`], such as
/// [`ReadRelation::query_all_handler()`], use Axum state extractors to provide the database
/// connection they need to actually perform operations on the database without reconnecting to it
/// every time. Any state type that implements this trait only needs to encapsulate a
/// [`PgDatabase`], but can contain any other data as is necessary.
pub trait DatabaseState {
    /// Get the inner [`PgDatabase`] from this state type.
    fn get_database(&self) -> PgDatabase;
    /// Get the inner [`PgDatabase::connection`] from this state type.
    fn get_database_connection(&self) -> sqlx::PgPool;
}

#[derive(Clone)]
pub struct PgDatabase {
    pub connection: sqlx::PgPool,
}

impl PgDatabase {
    pub async fn execute_query_builder<'a>(&self, mut query_builder: QueryBuilder<'a, Postgres>) {
        query_builder
            .build()
            .execute(&self.connection)
            .await
            .unwrap();
    }
}
