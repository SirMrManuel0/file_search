use std::{io::Read, path::Path};
use std::fs::File;
use anyhow;

use rusqlite::{Connection, Error, };

use crate::configuration::structs::DatabaseConfiguration;

pub fn create_connection(path: &str) -> Result<Connection, Error> {
    let conn: Connection = Connection::open(path)?;
    conn.execute("PRAGMA foreign_keys = ON;", ())?;
    Ok(conn)
}

pub fn create_tables(connection: &Connection, database_configuration: &DatabaseConfiguration) -> anyhow::Result<()> {
    create_tables_transaction(connection, Path::new(&database_configuration.creation_transaction))
}

pub fn create_tables_management(connection: &Connection, database_configuration: &DatabaseConfiguration) -> anyhow::Result<()> {
    create_tables_transaction(connection, Path::new(&database_configuration.creation_transaction_management))
}

fn create_tables_transaction(connection: &Connection, path: &Path) -> anyhow::Result<()> {
    let creation_transaction_path: &Path = path;
    let mut creation_transaction_file: File = File::open(creation_transaction_path)?;
    let mut creation_transaction: String = String::new();
    creation_transaction_file.read_to_string(&mut creation_transaction)?;

    connection.execute_batch(&creation_transaction)?;

    Ok(())
}