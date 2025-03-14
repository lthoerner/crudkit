use http::StatusCode;
use sqlx::Error as SqlxError;

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    BrokenDatabaseConnection,
    InvalidQuery,
    UnexpectedQueryResult,
}

impl From<SqlxError> for Error {
    fn from(value: SqlxError) -> Self {
        match value {
            SqlxError::Configuration(_)
            | SqlxError::Io(_)
            | SqlxError::Tls(_)
            | SqlxError::Protocol(_)
            | SqlxError::AnyDriverError(_)
            | SqlxError::PoolTimedOut
            | SqlxError::PoolClosed
            | SqlxError::WorkerCrashed
            | SqlxError::Database(_) => Self::BrokenDatabaseConnection,
            SqlxError::TypeNotFound { .. }
            | SqlxError::ColumnIndexOutOfBounds { .. }
            | SqlxError::ColumnNotFound(_)
            | SqlxError::Encode(_) => Self::InvalidQuery,
            SqlxError::RowNotFound | SqlxError::Decode(_) => Self::UnexpectedQueryResult,
            _ => todo!(),
        }
    }
}

impl From<Error> for StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::BrokenDatabaseConnection | Error::UnexpectedQueryResult => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::InvalidQuery => StatusCode::BAD_REQUEST,
        }
    }
}
