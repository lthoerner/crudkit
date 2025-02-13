use crate::database::Database;

// TODO: This will need to be replaced with a generic
#[derive(Clone)]
pub struct ServerState {
    pub database: Database,
}
