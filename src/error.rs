use http::StatusCode;
use sqlx::Error as SqlxError;

pub(crate) type Result<T> = core::result::Result<T, Error>;

// TODO: Implement `Error` trait
/// The Crudkit error type.
///
/// It is recommended that you alias this error type when importing it, in order to avoid confusion
/// with the many other types and traits named `Error`. This can be done like so:
/// ```rs
/// use crudkit::error::Error as CrudkitError;
/// ```
#[derive(Debug)]
pub struct Error {
    /// The general category of the error.
    ///
    /// This does not provide the exact underlying cause of an error, only a general category
    /// denoting what happened. If you care about the exact error, refer to [`Error::source`]
    /// instead. For user-facing errors, [`Error::status_code`] should be sufficient.
    pub kind: ErrorKind,
    /// The underlying [`sqlx`] error that the error was mapped from, if applicable.
    ///
    /// Most of the time when an [`Error`] is returned, it is created as an interpretation of a
    /// [`sqlx::Error`]. There are some situations where an [`Error`] is constructed manually
    /// outside of a [`sqlx`] context, in which this value will be [`None`].
    pub source: Option<SqlxError>,
    /// The HTTP status code corresponding to the error.
    ///
    /// [`Error`] is exposed directly in the return types of the read/write functions for records
    /// and relations. In the Axum handler versions of the functions, it is converted to a
    /// [`StatusCode`] in order to be returned as an [`axum::response::Response`]. These
    /// [`StatusCode`] mappings are relatively basic and are subject to change in the future.
    pub status_code: StatusCode,
}

/// A set of broad categories used by [`Error`].
///
/// This is not an exhaustive list and is subject to change in the future. It is meant to provide
/// surface-level information that can be handled simply should an error need to be handled
/// explicitly, but in most cases, errors should be propagated to the user via
/// [`Error::status_code`].
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The database is no longer accessible because of a connection or authentication failure.
    BrokenDatabaseConnection,
    /// The query sent to the database either was malformed/syntactically invalid or could not be
    /// serialized.
    InvalidQuery,
    /// The database returned an unexpected result based on the operation performed, i.e. returning
    /// no rows when expected to return a single row.
    UnexpectedQueryResult,
}

impl From<SqlxError> for Error {
    fn from(source_error: SqlxError) -> Self {
        match &source_error {
            SqlxError::Configuration(_)
            | SqlxError::Io(_)
            | SqlxError::Tls(_)
            | SqlxError::Protocol(_)
            | SqlxError::AnyDriverError(_)
            | SqlxError::PoolTimedOut
            | SqlxError::PoolClosed
            | SqlxError::WorkerCrashed
            | SqlxError::Database(_) => Self {
                kind: ErrorKind::BrokenDatabaseConnection,
                source: Some(source_error),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            SqlxError::TypeNotFound { .. }
            | SqlxError::ColumnIndexOutOfBounds { .. }
            | SqlxError::ColumnNotFound(_)
            | SqlxError::Encode(_) => Self {
                kind: ErrorKind::InvalidQuery,
                source: Some(source_error),
                status_code: StatusCode::BAD_REQUEST,
            },
            SqlxError::RowNotFound => Self {
                kind: ErrorKind::UnexpectedQueryResult,
                source: Some(source_error),
                status_code: StatusCode::NOT_FOUND,
            },
            SqlxError::Decode(_) => Self {
                kind: ErrorKind::UnexpectedQueryResult,
                source: Some(source_error),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => todo!(),
        }
    }
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        error.status_code
    }
}
