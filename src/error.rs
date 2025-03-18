use http::StatusCode;
use sqlx::Error as SqlxError;

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    pub kind: ErrorKind,
    pub source: SqlxError,
    status_code: StatusCode,
}

pub enum ErrorKind {
    BrokenDatabaseConnection,
    InvalidQuery,
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
                source: source_error,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            SqlxError::TypeNotFound { .. }
            | SqlxError::ColumnIndexOutOfBounds { .. }
            | SqlxError::ColumnNotFound(_)
            | SqlxError::Encode(_) => Self {
                kind: ErrorKind::InvalidQuery,
                source: source_error,
                status_code: StatusCode::BAD_REQUEST,
            },
            SqlxError::RowNotFound => Self {
                kind: ErrorKind::UnexpectedQueryResult,
                source: source_error,
                status_code: StatusCode::NOT_FOUND,
            },
            SqlxError::Decode(_) => Self {
                kind: ErrorKind::UnexpectedQueryResult,
                source: source_error,
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
