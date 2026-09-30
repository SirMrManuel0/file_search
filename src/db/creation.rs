use rusqlite::{Connection, Error, };

pub fn create_connection(path: &str) -> Result<Connection, Error> {
    let conn: Connection = Connection::open(path)?;
    Ok(conn)
}

pub fn create_tables(connection: &Connection) -> Result<(), Error> {
    connection.execute(
        "CREATE TABLE item (
            item_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            red INTEGER NOT NULL,
            green INTEGER NOT NULL,
            blue INTEGER NOT NULL,
            is_directory INTEGER NOT NULL CHECK (is_directory IN (0, 1))
        )",
        (),
    )?;

    connection.execute(
        "CREATE TABLE tag (
            tag_id INTEGER PRIMARY KEY,
            tag_name TEXT NOT NULL UNIQUE,
            red INTEGER NOT NULL,
            green INTEGER NOT NULL,
            blue INTEGER NOT NULL
        )",
        ()
    )?;

    connection.execute(
        "CREATE TABLE category (
            category_id INTEGER PRIMARY KEY,
            category_name TEXT NOT NULL UNIQUE,
            red INTEGER NOT NULL,
            green INTEGER NOT NULL,
            blue INTEGER NOT NULL
        )",
        ()
    )?;

    connection.execute(
        "CREATE TABLE color (
            color_id INTEGER PRIMARY KEY,
            red INTEGER,
            green INTEGER,
            blue INTEGER,
            alpha INTEGER
        )",
        ()
    )?;

    connection.execute(
        "CREATE TABLE item_tag_index (
            item_id INTEGER,
            tag_id INTEGER,
            PRIMARY KEY (item_id, tag_id)
        )",
        ()
    )?;

    connection.execute(
        "CREATE TABLE item_category_index (
            item_id INTEGER,
            category_id INTEGER,
            PRIMARY KEY (item_id, category_id)
        )",
        ()
    )?;

    connection.execute(
        "CREATE TABLE tag_linked_tag_index (
            tag_id INTEGER,
            linked_tag_id INTEGER,
            PRIMARY KEY (tag_id, linked_tag_id)
        )",
        ()
    )?;

    connection.execute(
        "CREATE TABLE tag_category_index (
            tag_id INTEGER,
            category_id INTEGER,
            PRIMARY KEY (tag_id, category_id)
        )",
        ()
    )?;

    Ok(())
}