use rusqlite::Connection;
use file_search::configuration::Configuration;
use file_search::db::creation::{create_tables, create_tables_management};
use file_search::db::insert;
use file_search::db::structs::{Color, FileSearchDatabase};

mod common;

fn create_database_connection() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", ()).unwrap();
    conn
}

fn create_tables_common() -> Connection{
    let connection: Connection = create_database_connection();
    let configuration: Configuration = common::get_configuration();
    create_tables(&connection, &configuration.database).unwrap();
    connection
}

fn create_tables_management_common() -> Connection{
    let connection: Connection = create_database_connection();
    let configuration: Configuration = common::get_configuration();
    create_tables_management(&connection, &configuration.database).unwrap();
    connection
}

pub fn populate_table_color(connection: &Connection) {
    let test_data: Vec<Color> = vec![
        Color { id: 0, name: String::from("Red"),    description: String::from("Pure red"),    red: 255, green: 0,   blue: 0,   alpha: 255 },
        Color { id: 0, name: String::from("Green"),  description: String::from("Pure green"),  red: 0,   green: 255, blue: 0,   alpha: 255 },
        Color { id: 0, name: String::from("Blue"),   description: String::from("Pure blue"),   red: 0,   green: 0,   blue: 255, alpha: 255 },
        Color { id: 0, name: String::from("White"),  description: String::from("All channels at max"), red: 255, green: 255, blue: 255, alpha: 255 },
        Color { id: 0, name: String::from("Black"),  description: String::from("All colour channels zero"), red: 0, green: 0, blue: 0, alpha: 255 },
        Color { id: 0, name: String::from("Orange"), description: String::from("Warm orange"), red: 255, green: 165, blue: 0, alpha: 255 },

        Color { id: 0, name: String::from("Transparent"), description: String::from("Fully transparent"), red: 0, green: 0, blue: 0, alpha: 0 },
        Color { id: 0, name:String::from("Half transparent red"), description: String::from("50% alpha"), red: 255, green: 0, blue: 0, alpha: 128 },
        Color { id: 0, name: String::from("Min"), description: String::from("Everything 0"), red: 0, green: 0, blue: 0, alpha: 0 },
        Color { id: 0, name: String::from("Max"), description: String::from("Everything 255"), red: 255, green: 255, blue: 255, alpha: 255 },
        Color { id: 0, name: String::from("Off by one"), description: String::from("Values next to the limits"), red: 1, green: 254, blue: 127, alpha: 128 },

        Color { id: 0, name: String::from(""), description: String::from(""), red: 10, green: 20, blue: 30, alpha: 40 },
        Color { id: 0, name: String::from("   "), description: String::from("Only spaces"), red: 1, green: 1, blue: 1, alpha: 1 },
        Color { id: 0, name: String::from("Dunkelgrün 🌲"), description: String::from("Äöü ß 日本語\nZweite Zeile\tTab"), red: 1, green: 100, blue: 32, alpha: 255 },
        Color { id: 0, name: String::from("Doppelt"), description: String::from("Erster Eintrag mit gleichem Namen"), red: 5, green: 5, blue: 5, alpha: 255 },
        Color { id: 0, name: String::from("Doppelt"), description: String::from("Zweiter Eintrag mit gleichem Namen"), red: 6, green: 6, blue: 6, alpha: 255 },
        Color { id: 0, name: "n".repeat(10_000), description: "d".repeat(100_000), red: 9, green: 9, blue: 9, alpha: 9 },

        // --- SQL-Injection ---
        Color { id: 0, name: String::from("'; DROP TABLE color; --"), description: String::from("Bobby Tables"), red: 1, green: 2, blue: 3, alpha: 4 },
        Color { id: 0, name: String::from("' OR 1=1 --"), description: String::from("' OR '1'='1"), red: 5, green: 6, blue: 7, alpha: 8 },
        Color { id: 0, name: String::from("x'); DELETE FROM color; --"), description: String::from("') OR ('1'='1"), red: 9, green: 10, blue: 11, alpha: 12 },
        Color { id: 0, name: String::from("' UNION SELECT * FROM databases --"), description: String::from("UNION attack"), red: 13, green: 14, blue: 15, alpha: 16 },
        Color { id: 0, name: String::from("\" OR \"\"=\""), description: String::from("Doppelte Anführungszeichen"), red: 17, green: 18, blue: 19, alpha: 20 },
        Color { id: 0, name: String::from("\\'; DROP TABLE databases; --"), description: String::from("Backslash-Escape-Versuch"), red: 21, green: 22, blue: 23, alpha: 24 },
        Color { id: 0, name: String::from("a'; INSERT INTO color (color_name, color_description) VALUES ('evil','evil'); --"),
            description: String::from("/* Kommentar */ ' --"), red: 25, green: 26, blue: 27, alpha: 28 },
        Color { id: 0, name: String::from("'||(SELECT name FROM sqlite_master LIMIT 1)||'"), description: String::from("%' OR color_name LIKE '%"), red: 29, green: 30, blue: 31, alpha: 32 },
        Color { id: 0, name: String::from("1; ATTACH DATABASE 'evil.db' AS e; --"), description: String::from("'; PRAGMA foreign_keys = OFF; --"), red: 33, green: 34, blue: 35, alpha: 36 },
    ];

    for data in test_data{
        insert::insert_color(
            connection,
            data.name,
            data.description,
            data.red,
            data.green,
            data.blue,
            data.alpha
        ).unwrap();
    }

}

pub fn populate_tables_management() -> Connection {
    let connection: Connection = create_tables_management_common();
    populate_table_color(&connection);

    let test_data: Vec<FileSearchDatabase> = vec![
        FileSearchDatabase {
            id: 0, name: String::from("Hello"), description: String::from("Some description"),
            location: String::from("/this/is/a/path"), is_in_cloud: true, is_ssh: true,
            username: Some(String::from("this is a username")),
            password_encrypted: Some(String::from("Encrypted Password")),
            location_private_key: Some(String::from("/this/is/a/path/to/a/private/key")),
            server_address: Some(String::from("127.0.0.1")), color_id: 1,
        },

        FileSearchDatabase {
            id: 0, name: String::from("Lokal"), description: String::from("Nur lokal"),
            location: String::from(r"C:\Users\kumar\data.db"), is_in_cloud: false, is_ssh: false,
            username: None, password_encrypted: None,
            location_private_key: None, server_address: None, color_id: 1,
        },

        FileSearchDatabase {
            id: 0, name: String::from("Cloud"), description: String::from("Cloud ohne SSH"),
            location: String::from("https://example.com/db"), is_in_cloud: true, is_ssh: false,
            username: Some(String::from("admin")), password_encrypted: Some(String::from("x9f3k2")),
            location_private_key: None, server_address: Some(String::from("db.example.com")), color_id: 1,
        },

        FileSearchDatabase {
            id: 0, name: String::from(""), description:String::from(""), location: String::from(""),
            is_in_cloud: false, is_ssh: false,
            username: Some(String::from("")), password_encrypted: Some(String::from("")),
            location_private_key: Some(String::from("")), server_address: Some(String::from("")), color_id: 1,
        },

        FileSearchDatabase {
            id: 0, name: String::from("Äöü ß 日本語 🚀"), description: String::from("Zeile 1\nZeile 2\tTab"),
            location: String::from("/pfad mit/leerzeichen/Ünïcödé"), is_in_cloud: false, is_ssh: true,
            username: Some(String::from("  user mit spaces  ")), password_encrypted: Some(String::from("pässwörd🔒")),
            location_private_key: Some(String::from("/schlüssel/id_rsa")), server_address: Some(String::from("::1")), color_id: 1,
        },

        FileSearchDatabase {
            id: 0, name: "a".repeat(10_000), description: "b".repeat(100_000),
            location: "/x".repeat(5_000), is_in_cloud: true, is_ssh: true,
            username: Some("u".repeat(1_000)), password_encrypted: Some("p".repeat(1_000)),
            location_private_key: Some("/k".repeat(2_000)), server_address: Some("h".repeat(255)), color_id: 1,
        },

        // ---- SQL-Injection ----

        FileSearchDatabase {
            id: 0, name: String::from("'; DROP TABLE databases; --"), description: String::from("Bobby Tables"),
            location: String::from("/tmp'); DROP TABLE color; --"), is_in_cloud: false, is_ssh: false,
            username: Some(String::from("admin'--")), password_encrypted: Some(String::from("' OR '1'='1")),
            location_private_key: None, server_address: None, color_id: 1,
        },

        FileSearchDatabase {
            id: 0, name: String::from("' OR 1=1 --"), description: String::from("' UNION SELECT * FROM color --"),
            location: String::from("\" OR \"\"=\""), is_in_cloud: true, is_ssh: true,
            username: Some(String::from("' OR ''='")), password_encrypted: Some(String::from("') OR ('1'='1")),
            location_private_key: Some(String::from("'; SELECT sqlite_version(); --")),
            server_address: Some(String::from("127.0.0.1'; DELETE FROM databases; --")), color_id: 1,
        },

        FileSearchDatabase {
            id: 0, name: String::from("x'; INSERT INTO color VALUES (999,'evil','evil',0,0,0,0); --"),
            description: String::from("/* kommentar */ ' --"), location: String::from("\\'; DROP TABLE databases;--"),
            is_in_cloud: false, is_ssh: true,
            username: Some(String::from("'; PRAGMA foreign_keys = OFF; --")),
            password_encrypted: Some(String::from("%' OR name LIKE '%")),
            location_private_key: Some(String::from("'||(SELECT name FROM sqlite_master)||'")),
            server_address: Some(String::from("1; ATTACH DATABASE 'evil.db' AS e; --")), color_id: 1,
        },
    ];

    for data in test_data{
        insert::insert_database(
            &connection,
            &data.name,
            &data.description,
            &data.location,
            data.color_id,
            data.is_in_cloud,
            data.is_ssh,
            data.username,
            data.server_address,
            data.location_private_key,
            data.password_encrypted
        ).unwrap();
    }

    connection
}

#[test]
fn test_create_tables() {
    create_tables_common();
}

#[test]
fn test_create_tables_management() {
    create_tables_management_common();
}

#[test]
fn test_insert_color_table() {
    let connection: Connection = create_tables_common();
    populate_table_color(&connection);
}

#[test]
fn test_insert_color_table_management() {
    let connection: Connection = create_tables_management_common();
    populate_table_color(&connection);
}

#[test]
fn test_insert_tables_management() {
    populate_tables_management();
}