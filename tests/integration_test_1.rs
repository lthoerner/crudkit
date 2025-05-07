// TODO: creating a module like this is a little messy, refactor this to something better in the
// future.
#[path = "./database_connection.rs"]
mod database_connection;

use serde::Serialize;

use crudkit::{
    traits::{
        id_parameter::{GenericIdParameter, IdParameter},
        read::ReadRelation,
        write::{BulkInsert, SingleInsert, WriteRelation},
    },
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};
use database_connection::get_database;
use serial_test::serial;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Clone, Serialize)]
#[relation(relation_name = "customers", primary_key = "id")]
pub struct CustomersTable {
    records: Vec<CustomersTableRecord>,
}

#[derive(
    Record,
    ReadRecord,
    WriteRecord,
    SingleInsert,
    IdentifiableRecord,
    sqlx::FromRow,
    Clone,
    Serialize,
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
#[serial(customers_table)]
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

    new_record
        .insert(&database)
        .await
        .expect("customers record creation failed");

    let record = CustomersTable::query_one(&database, id_parameter.clone())
        .await
        .expect("customers record query failed");

    assert_eq!(record.id, Some(id as i32));
    assert_eq!(record.name, "John Doe".to_string());

    CustomersTable::delete_one(&database, id_parameter)
        .await
        .expect("customers record deletion failed");
}

#[tokio::test]
#[serial(customers_table)]
async fn update_one_should_work() {
    let id = 2;
    let id_parameter = GenericIdParameter::new(id);
    let new_record = CustomersTableRecord {
        id: Some(id as i32),
        name: "Jane Doe".to_string(),
        email_address: Some("janedoe@gmail.com".to_string()),
        phone_number: None,
        street_address: None,
    };

    let database = get_database().await;

    new_record
        .insert(&database)
        .await
        .expect("customers record creation failed");

    let record = CustomersTable::query_one(&database, id_parameter.clone())
        .await
        .expect("customers record query failed");

    assert_eq!(record.id, Some(id as i32));
    assert_eq!(record.name, "Jane Doe".to_string());
    assert_eq!(record.email_address, Some("janedoe@gmail.com".to_string()));
    assert_eq!(record.phone_number, None);
    assert_eq!(record.street_address, None);

    let updated_record = CustomersTableRecordUpdateQueryParameters {
        id: record.id,             // ID is required else `update_one` will fail
        name: None,                // Do not change name
        email_address: Some(None), // Set email to `None`
        phone_number: Some(Some("1234567890".to_string())), // Add value
        street_address: Some(Some("123 Some street East".to_string())), // Add value
    };
    CustomersTable::update_one(&database, updated_record)
        .await
        .expect("customers record update failed");

    let updated_record = CustomersTable::query_one(&database, id_parameter.clone())
        .await
        .expect("customers record query failed");

    assert_eq!(updated_record.id, Some(id as i32));
    assert_eq!(updated_record.name, "Jane Doe".to_string());
    assert_eq!(updated_record.email_address, None);
    assert_eq!(updated_record.phone_number, Some("1234567890".to_string()));
    assert_eq!(
        updated_record.street_address,
        Some("123 Some street East".to_string())
    );

    CustomersTable::delete_one(&database, id_parameter)
        .await
        .expect("customers record deletion failed");
}

#[tokio::test]
#[serial(customers_table)]
async fn bulk_insert_query_all_and_delete_all_should_work() {
    let customers = (0..10)
        .map(|i| CustomersTableRecord {
            id: Some(i),
            name: format!("John Doe {i}").to_string(),
            email_address: Some("jdoe@email.com".to_string()),
            phone_number: Some("1234567890".to_string()),
            street_address: Some("123 Some street East".to_string()),
        })
        .collect();
    let customers_table = CustomersTable { records: customers };

    let database = get_database().await;

    customers_table
        .insert_all(&database)
        .await
        .expect("customers table creation failed");

    let records = CustomersTable::query_all(&database)
        .await
        .expect("customers table query failed")
        .records;

    assert_eq!(records.len(), 10);
    assert_eq!(records[0].id, Some(0));
    assert_eq!(records[0].name, "John Doe 0".to_string());
    assert_eq!(records[9].id, Some(9));
    assert_eq!(records[9].name, "John Doe 9".to_string());

    CustomersTable::delete_all(&database)
        .await
        .expect("customers table deletion failed");
}
