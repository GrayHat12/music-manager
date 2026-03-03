use diesel::prelude::*;
use dotenvy::dotenv;

pub mod commands;
pub mod models;
pub mod schema;
pub mod types;
pub mod utils;

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_connection(db_path: String) -> SqliteConnection {
    dotenv().ok();
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

pub fn initialise_connection(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
