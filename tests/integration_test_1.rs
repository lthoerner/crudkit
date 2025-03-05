use crudkit::{
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Clone)]
#[relation(relation_name = "customers", primary_key = "id")]
pub struct CustomersTable {
    records: Vec<CustomersTableRecord>,
}

#[derive(
    Record, ReadRecord, WriteRecord, SingleInsert, IdentifiableRecord, sqlx::FromRow, Clone,
)]
pub struct CustomersTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}
