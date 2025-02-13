use crate::database::Database;

#[derive(Clone)]
pub struct ServerState {
    pub database: Database,
    imei_info_api_key: String,
}
