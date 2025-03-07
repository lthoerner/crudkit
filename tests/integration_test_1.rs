// TODO: creating a module like this is a little messy, refactor this to
// something better in the future.
#[path = "./database_connection.rs"]
mod database_connection;

use crudkit::{
    traits::{
        id_parameter::{GenericIdParameter, IdParameter},
        read::ReadRelation,
        write::{SingleInsert, WriteRelation},
    },
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};
use database_connection::get_database;

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

#[tokio::test]
async fn insert_query_one_and_delete_one_should_work() {
    let id = 1;
    let id_parameter = GenericIdParameter::new(id);
    let new_record = CustomersTableRecord {
        id: Some(id as i32),
        name: "John Doe".to_string(),
        email_address: Some("jdoe@email.com".to_string()),
        phone_number: Some("1234567890".to_string()),
        street_address: Some("123 Some street East".to_string()),
    };

    let database = get_database().await;

    new_record.insert(&database).await;

    let record = CustomersTable::query_one(&database, id_parameter)
        .await
        .expect("query to contain record");

    assert_eq!(record.id, Some(id as i32));
    assert_eq!(record.name, "John Doe".to_string());

    CustomersTable::delete_one(&database, id_parameter).await;
}
