use std::future::Future;
use std::sync::Arc;

use axum::extract::{Query, State};
use http::StatusCode;
use sqlx::query_builder::{QueryBuilder, Separated};
use sqlx::Postgres;

use super::id_parameter::IdParameter;
#[allow(unused_imports)]
use super::read::{ReadRecord, ReadRelation};
use super::shared::{Record, Relation};
use crate::database::{DatabaseState, PgDatabase, SQL_PARAMETER_BIND_LIMIT};
use crate::error::{Error as CrudkitError, Result as CrudkitResult};

/// A trait that enables writable tables to have their records modified in the database.
///
/// This trait and [`ReadRelation`] are separated because because "relations" can be views, which
/// are read-only. For view types, only [`ReadRelation`] should be implemented. For table types,
/// both traits can be implemented safely.
///
/// This trait gets most of the information it needs to function from the upstream [`Relation`]
/// trait.
///
/// Also see [`WriteRecord`].
pub trait WriteRelation: Relation {
    /// The record type which this relation contains a collection of.
    ///
    /// This type and the [`WriteRecord::WriteRelation`] type are directly interreferential to allow
    /// convenient "upcasting" so record types can be used interchangeably with relation types.
    ///
    /// This type is declared separately from [`Relation::Record`] because of cyclic dependency
    /// issues, but the type it refers to must be the same.
    type WriteRecord: WriteRecord<WriteRelation = Self>;

    /// Create a single record in the database.
    ///
    /// In the future, this will return a proper status code. At the moment, it does not return
    /// anything because the underlying [`SingleInsert::insert()`] does not implement error
    /// handling.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`WriteRelation::create_one_handler()`].
    // * This method does not emit any logs because `SingleInsert::insert()` already emits logs.
    fn create_one(
        database: &PgDatabase,
        create_params: <Self::WriteRecord as WriteRecord>::CreateQueryParameters,
    ) -> impl Future<Output = CrudkitResult<()>> + Send {
        async { create_params.into().insert(database).await }
    }

    /// Create a single record in the database.
    ///
    /// In the future, this will return a proper status code. At the moment, it just returns a
    /// placeholder status code because the underlying [`SingleInsert::insert()`] does not implement
    /// error handling.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`WriteRelation::create_one()`].
    fn create_one_handler<S: DatabaseState>(
        state: State<Arc<S>>,
        Query(create_params): Query<<Self::WriteRecord as WriteRecord>::CreateQueryParameters>,
    ) -> impl Future<Output = StatusCode> + Send {
        async move {
            let relation_name = Self::get_qualified_name();
            log::debug!(
                "Request received by single-CREATE endpoint for relation {relation_name}, calling
                query dispatcher"
            );

            match Self::create_one(state.get_database(), create_params).await {
                Ok(_) => StatusCode::CREATED,
                Err(e) => StatusCode::from(e),
            }
        }
    }

    /// Update a single record in the database.
    ///
    /// In the future, this will return a proper status code. At the moment, it just returns a
    /// placeholder status code because the underlying [`WriteRecord::update_one()`] does not
    /// implement error handling.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`WriteRelation::update_one_handler()`].
    fn update_one(
        database: &PgDatabase,
        update_params: <Self::WriteRecord as WriteRecord>::UpdateQueryParameters,
    ) -> impl Future<Output = CrudkitResult<()>> + Send {
        let relation_name = Self::get_qualified_name();
        log::debug!(
            "Dispatching single-UPDATE query to database, targeting relation {relation_name}"
        );

        <Self::WriteRecord as WriteRecord>::update_one(database, update_params)
    }

    /// Update a single record in the database.
    ///
    /// In the future, this will return a proper status code. At the moment, it just returns a
    /// placeholder status code because the underlying [`WriteRecord::update_one()`] does not
    /// implement error handling.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`WriteRelation::update_one()`].
    fn update_one_handler<S: DatabaseState>(
        state: State<Arc<S>>,
        Query(update_params): Query<<Self::WriteRecord as WriteRecord>::UpdateQueryParameters>,
    ) -> impl Future<Output = StatusCode> + Send {
        async move {
            let relation_name = Self::get_qualified_name();
            log::debug!(
                "Request received by single-UPDATE endpoint for relation {relation_name}, calling
                query dispatcher"
            );

            match Self::update_one(state.get_database(), update_params).await {
                Ok(_) => StatusCode::OK,
                Err(e) => StatusCode::from(e),
            }
        }
    }

    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`WriteRelation::delete_one_handler()`].
    fn delete_one<I: IdParameter>(
        database: &PgDatabase,
        id: I,
    ) -> impl Future<Output = CrudkitResult<()>> + Send {
        async move {
            let relation_name = Self::get_qualified_name();
            let query_string = format!(
                "DELETE FROM {}.{} WHERE {} = $1",
                Self::SCHEMA_NAME,
                Self::RELATION_NAME,
                Self::PRIMARY_KEY,
            );

            log::debug!(
                "Dispatching single-DELETE query to database, targeting relation {relation_name}"
            );
            log::trace!("Raw query: {query_string}");

            match sqlx::query(&query_string)
                .bind(id.id() as i32)
                .execute(&database.connection)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(CrudkitError::from(e)),
            }
        }
    }

    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`WriteRelation::delete_one()`].
    fn delete_one_handler<I: IdParameter, S: DatabaseState>(
        state: State<Arc<S>>,
        Query(id_param): Query<I>,
    ) -> impl Future<Output = StatusCode> + Send {
        async move {
            let relation_name = Self::get_qualified_name();
            log::debug!(
                "Request received by single-DELETE endpoint for relation {relation_name}, calling
                query dispatcher"
            );

            match Self::delete_one(state.get_database(), id_param).await {
                Ok(_) => StatusCode::OK,
                Err(e) => StatusCode::from(e),
            }
        }
    }

    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`WriteRelation::delete_all_handler()`].
    fn delete_all(database: &PgDatabase) -> impl Future<Output = CrudkitResult<()>> + Send {
        async move {
            let relation_name = Self::get_qualified_name();
            let query_string = format!("DELETE FROM {}.{}", Self::SCHEMA_NAME, Self::RELATION_NAME);

            log::debug!(
                "Dispatching multi-DELETE query to database, targeting relation {relation_name}"
            );
            log::trace!("Raw query: {query_string}");

            match sqlx::query(&query_string)
                .execute(&database.connection)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(CrudkitError::from(e)),
            }
        }
    }

    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`WriteRelation::delete_all()`].
    fn delete_all_handler<S: DatabaseState>(
        state: State<Arc<S>>,
    ) -> impl Future<Output = StatusCode> + Send {
        async move {
            let relation_name = Self::get_qualified_name();
            log::debug!(
                "Request received by multi-DELETE endpoint for relation {relation_name}, calling
                query dispatcher"
            );

            match Self::delete_all(state.get_database()).await {
                Ok(_) => StatusCode::OK,
                Err(e) => StatusCode::from(e),
            }
        }
    }
}

/// A trait that enables writable tables to have their records modified in the database.
///
/// This trait and [`ReadRecord`] are separated because because "relations" can be views, which
/// are read-only. For view record types, only [`ReadRecord`] should be implemented. For table
/// record types, both traits can be implemented safely.
///
/// This trait gets most of the information it needs to function from the upstream [`Relation`]
/// trait, using the associated type [`Record::Relation`].
///
/// Also see [`WriteRelation`].
pub trait WriteRecord: Record<Relation: WriteRelation> + SingleInsert {
    /// The relation type which contains a collection of this record type.
    ///
    /// This type and the [`WriteRelation::WriteRecord`] type are directly interreferential to allow
    /// convenient "upcasting" so record types can be used interchangeably with relation types.
    ///
    /// This type is declared separately from [`Record::Relation`] because of cyclic dependency
    /// issues, but the type it refers to must be the same.
    type WriteRelation: WriteRelation<WriteRecord = Self>;
    // * Both of the following types require a `Clone` and `Deserialize` implementation to work, but
    // * since `Deserialize` requires lifetime annotations to be added everywhere, they are left out
    // * of the trait bounds and instead simply added to the `WriteRecord` derive macro.
    /// A type used for deserializing the query parameters in a request to a CREATE endpoint, which
    /// includes all of the table's columns as fields except ID fields that are auto-generated in
    /// the database.
    type CreateQueryParameters: Into<Self> + Send + Sync;
    /// A type used for deserializing the query parameters in a request to an UPDATE endpoint, which
    /// includes all of the table's columns as optional fields except ID fields that must be
    /// specified for the database to determine which record to update.
    type UpdateQueryParameters: Send + Sync;

    /// Update a single record in the database.
    ///
    /// This method is used by [`WriteRelation::update_one()`] because the [`WriteRelation`] derive
    /// macro does not have access to the field names and primary keys of the record type, which it
    /// would need to generate this implementation. In the future, this will likely be fixed by
    /// using a module-wide macro rather than multiple type-level macros. For now, it is recommended
    /// to use [`WriteRelation`]'s version of these methods.
    fn update_one(
        database: &PgDatabase,
        update_params: Self::UpdateQueryParameters,
    ) -> impl Future<Output = CrudkitResult<()>> + Send;
}

/// A trait that allows a single record to be inserted to the database.
///
/// Though it would be possible to make this trait generic over [`Record`], it is only meant to be
/// implemented on [`WriteRecord`] types, as items cannot be inserted into a database view.
///
/// For bulk-insertion of records, see the related [`BulkInsert`] trait.
pub trait SingleInsert: Record {
    /// Get the [`QueryBuilder`] necessary to insert one or more records of data into the database.
    ///
    /// This is used by both [`SingleInsert`] and [`BulkInsert`] and is meant mostly for
    /// auto-implementations.
    fn get_query_builder<'a>() -> QueryBuilder<'a, Postgres> {
        QueryBuilder::new(&format!(
            "INSERT INTO {}.{} ({}) ",
            Self::Relation::SCHEMA_NAME,
            Self::Relation::RELATION_NAME,
            Self::COLUMN_NAMES.join(", ")
        ))
    }

    /// Push the record's data into the [`QueryBuilder`] so it can be built and executed against the
    /// database.
    ///
    /// This method is used as a function parameter for [`QueryBuilder::push_values`] and should
    /// only be used within auto-implementations.
    fn push_column_bindings(builder: Separated<Postgres, &str>, record: Self);

    /// Insert the record into the database.
    ///
    /// This should not be used repeatedly for a collection of records. Inserting multiple records
    /// can be done much more efficiently using [`BulkInsert::insert_all`], which should be
    /// implemented for any database table type.
    fn insert(self, database: &PgDatabase) -> impl Future<Output = CrudkitResult<()>> + Send {
        async move {
            let relation_name = Self::Relation::get_qualified_name();
            log::debug!(
                "Dispatching single-INSERT query to database, targeting relation {relation_name}"
            );

            let mut query_builder = Self::get_query_builder();
            query_builder.push_values(std::iter::once(self), Self::push_column_bindings);

            let query_string = query_builder.sql();
            log::trace!("Raw query: {query_string}");

            match query_builder.build().execute(&database.connection).await {
                Ok(_) => {
                    log::debug!("Data has been successfully inserted");
                    Ok(())
                }
                Err(e) => {
                    log::debug!("Failed to insert data to relation {relation_name}");
                    Err(CrudkitError::from(e))
                }
            }
        }
    }
}

/// A trait that allows an entire table of records to be inserted to the database in large batches.
///
/// Bulk-inserting items removes the need for establishing a network connection to the database
/// repeatedly. In initial testing, this proved to be about 20x more efficient than single insertion
/// when working with large tables. Of course, this is mostly used with synthetic data for testing
/// purposes, as it is relatively rare for a significant number of records to be inserted at once
/// during normal operation.
///
/// For single-insertion of records, see the related [`SingleInsert`] trait.
pub trait BulkInsert: WriteRelation<Record: SingleInsert> {
    /// The amount of records that can be inserted per batch/chunk.
    ///
    /// The batch limit is determined by the number of columns in a table. This is because a single
    /// SQL statement only supports up to [`u16::MAX`] parameter bindings, and each column takes up
    /// one parameter. Effectively, this means that tables with more columns are split into more
    /// batches, making bulk insertion take longer.
    const CHUNK_SIZE: usize = SQL_PARAMETER_BIND_LIMIT / Self::Record::COLUMN_NAMES.len();

    /// Convert a table of records into a series of batches to be inserted to the database.
    ///
    /// This method should only be used within auto-implementations.
    fn into_chunks(self) -> impl Iterator<Item = Vec<Self::Record>> + Send + Sync {
        let mut iter = self.take_records().into_iter();
        // TODO: Annotate this code or something, I have very little idea what it does
        // * This was done because `itertools::IntoChunks` was causing issues with the axum handlers
        std::iter::from_fn(move || Some(iter.by_ref().take(Self::CHUNK_SIZE).collect()))
            .take_while(|v: &Vec<_>| !v.is_empty())
    }

    /// Insert the entire table into the database in a series of batches (or "chunks").
    ///
    /// This can insert tables of arbitrary size, but each batch is limited in size by number of
    /// parameters (table column count * record count).
    fn insert_all(self, database: &PgDatabase) -> impl Future<Output = CrudkitResult<()>> + Send {
        async move {
            let relation_name = Self::get_qualified_name();
            log::debug!(
                "Dispatching multi-INSERT query to database, targeting relation {relation_name}"
            );

            let chunk_count = self.records().len() / Self::CHUNK_SIZE;
            for (i, chunk) in self.into_chunks().enumerate() {
                log::debug!("Inserting data chunk {i} of {chunk_count}");

                let mut query_builder = Self::Record::get_query_builder();
                query_builder.push_values(chunk, Self::Record::push_column_bindings);

                let query_string = query_builder.sql();
                log::trace!("Raw query: {query_string}");

                if let Err(e) = query_builder.build().execute(&database.connection).await {
                    log::error!(
                        "Failed to insert data chunk {i} of {chunk_count} to relation 
                        {relation_name}"
                    );
                    return Err(CrudkitError::from(e));
                }

                log::debug!("Data chunk has been successfully inserted");
            }

            log::debug!("All data chunks have been successfully inserted");

            Ok(())
        }
    }
}
