pub mod database;
pub mod error;
pub mod traits;

pub use axum;
pub use crudkit_derive::*;
pub use http;
pub use log;

pub mod prelude {
    pub use super::traits::id_parameter::{GenericIdParameter, IdParameter};
    pub use super::traits::read::{ReadRecord, ReadRelation};
    pub use super::traits::shared::{IdentifiableRecord, Record, Relation};
    pub use super::traits::write::{BulkInsert, SingleInsert, WriteRecord, WriteRelation};
    pub use crudkit_derive::IdParameter;
    pub use crudkit_derive::{BulkInsert, SingleInsert, WriteRecord, WriteRelation};
    pub use crudkit_derive::{IdentifiableRecord, Record, Relation};
    pub use crudkit_derive::{ReadRecord, ReadRelation};
}
