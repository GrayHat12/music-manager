use diesel::prelude::*;
use dotenvy::dotenv;

pub mod models;
pub mod schema;

pub fn establish_connection(db_path: String) -> SqliteConnection {
    dotenv().ok();
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}
