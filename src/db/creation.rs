use std::{io::Read, path::Path};
use std::fs::File;

use rusqlite::{Connection, Error, };

use crate::conf::structs::DatabaseConfiguration;

pub fn create_connection(path: &str) -> Result<Connection, Error> {
    let conn: Connection = Connection::open(path)?;
    conn.execute("PRAGMA foreign_keys = ON;", ())?;
    Ok(conn)
}

pub fn create_tables(connection: &Connection, database_configuration: &DatabaseConfiguration) -> Option<()> {
    let creation_transaction_path: &Path = Path::new(&database_configuration.creation_transaction);
    let mut creation_transaction_file: File = match File::open(creation_transaction_path) {
        Ok(file) => file,
        Err(_) => { return None; },
    };
    let mut creation_transaction: String = String::new();
    match creation_transaction_file.read_to_string(&mut creation_transaction) {
        Ok(_) => (),
        Err(_) => { return None; }
    };

    match connection.execute_batch(&creation_transaction) {
        Ok(_) => (),
        Err(_) => { return None; }
    };

    Some(())
}