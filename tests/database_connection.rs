use std::{env, future::Future};

use crudkit::database::PgDatabase;
use sqlx::{Connection, PgConnection};

pub fn get_database_connection_string() -> String {
    dotenvy::dotenv().unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let app_user = env::var("APP_USER").unwrap();
    let app_user_pwd = env::var("APP_USER_PWD").unwrap();
    let app_db_name = env::var("APP_DB_NAME").unwrap();
    let connection_string =
        format!("postgres://{app_user}:{app_user_pwd}@localhost:{db_port}/{app_db_name}");

    connection_string
}

pub fn get_database() -> impl Future<Output = PgDatabase> {
    async {
        PgDatabase {
            connection: sqlx::PgPool::connect(&get_database_connection_string())
                .await
                .unwrap(),
        }
    }
}

#[tokio::test]
async fn can_connect_to_database() {
    let connection_string = get_database_connection_string();
    let connection = PgConnection::connect(&connection_string).await.unwrap();
    connection.close().await.unwrap();
}
