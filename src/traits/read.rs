use std::future::Future;
use std::sync::Arc;

use axum::extract::{Json, Query, State};
use axum::response::{IntoResponse, Response};
use http::StatusCode;

use super::id_parameter::IdParameter;
use super::shared::{Record, Relation};
#[allow(unused_imports)]
use super::write::{WriteRecord, WriteRelation};
use crate::database::{DatabaseState, PgDatabase};
use crate::error::{Error as CrudkitError, Result as CrudkitResult};

/// A trait that enables readable tables and views to have their records queried from the database.
///
/// This trait and [`WriteRelation`] are separated because because "relations" can be views, which
/// are read-only. Writable table types should also implement [`WriteRelation`].
///
/// This trait gets most of the information it needs to function from the upstream [`Relation`]
/// trait.
///
/// Also see [`ReadRecord`].
pub trait ReadRelation: Relation {
    /// The record type which this relation contains a collection of.
    ///
    /// This type and the [`ReadRecord::ReadRelation`] type are directly interreferential to allow
    /// convenient "upcasting" so record types can be used interchangeably with relation types.
    ///
    /// This type is declared separately from [`Relation::Record`] because of cyclic dependency
    /// issues, but the type it refers to must be the same.
    type ReadRecord: ReadRecord<ReadRelation = Self>;

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`ReadRelation::query_one_handler()`].
    fn query_one<I: IdParameter>(
        database: &PgDatabase,
        id: I,
    ) -> impl Future<Output = CrudkitResult<Self::ReadRecord>> + Send {
        async move {
            match sqlx::query_as(&format!(
                "SELECT * FROM {}.{} WHERE {} = $1",
                Self::SCHEMA_NAME,
                Self::RELATION_NAME,
                Self::PRIMARY_KEY,
            ))
            .bind(id.id() as i32)
            .fetch_one(&database.connection)
            .await
            {
                Ok(record) => Ok(record),
                Err(e) => Err(CrudkitError::from(e)),
            }
        }
    }

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`ReadRelation::query_one()`].
    // TODO: Check how this interacts with junction tables
    fn query_one_handler<I: IdParameter, S: DatabaseState>(
        state: State<Arc<S>>,
        Query(id_param): Query<I>,
    ) -> impl Future<Output = Response> + Send {
        async move {
            match Self::query_one(state.get_database(), id_param).await {
                Ok(record) => Json(record).into_response(),
                Err(e) => StatusCode::from(e).into_response(),
            }
        }
    }

    /// Query (select) all records for this relation from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`ReadRelation::query_all_handler()`].
    fn query_all(database: &PgDatabase) -> impl Future<Output = CrudkitResult<Self>> + Send {
        async move {
            match sqlx::query_as(&format!(
                "SELECT * FROM {}.{} ORDER BY {}",
                Self::SCHEMA_NAME,
                Self::RELATION_NAME,
                Self::PRIMARY_KEY,
            ))
            .fetch_all(&database.connection)
            .await
            {
                Ok(records) => Ok(Self::with_records(records)),
                Err(e) => Err(CrudkitError::from(e)),
            }
        }
    }

    /// Query (select) all records for this relation from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`ReadRelation::query_all()`].
    fn query_all_handler<S: DatabaseState>(
        state: State<Arc<S>>,
    ) -> impl Future<Output = Response> + Send {
        async move {
            match Self::query_all(state.get_database()).await {
                Ok(records) => Json(records).into_response(),
                Err(e) => StatusCode::from(e).into_response(),
            }
        }
    }
}

/// A trait that enables readable tables and views to have their records queried from the database.
///
/// This trait and [`WriteRecord`] are separated because because "relations" can be views, which
/// are read-only. Writable table record types should also implement [`WriteRecord`].
///
/// This trait gets most of the information it needs to function from the upstream [`Relation`]
/// trait, using the associated type [`Record::Relation`].
///
/// Also see [`ReadRelation`].
pub trait ReadRecord: Record {
    /// The relation type which contains a collection of this record type.
    ///
    /// This type and the [`ReadRelation::ReadRecord`] type are directly interreferential to allow
    /// convenient "upcasting" so record types can be used interchangeably with relation types.
    ///
    /// This type is declared separately from [`Record::Relation`] because of cyclic dependency
    /// issues, but the type it refers to must be the same.
    type ReadRelation: ReadRelation<ReadRecord = Self>;
}
