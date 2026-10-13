use rusqlite::Connection;
use std::{io::Read, path::Path};
use std::fs::File;

use file_search::configuration::{Configuration, DatabaseConfiguration};
use file_search::db::creation::{create_tables, create_tables_management};
use crate::common;

pub fn get_configuration() -> Configuration {
    Configuration {
        title: "File Search Configuration".to_string(),
        database: DatabaseConfiguration {
            location: "database/managment_production.db3".to_string(),
            creation_transaction_management: "sql/management_database_creation.sql".to_string(),
            creation_transaction: "sql/table_creation.sql".to_string(),
        },
    }
}

