use std::fs::File;
use rusqlite::{Connection, Error, };

use crate::db::structs::{Item, Tag, Category, Color, FileSearchDatabase};

pub fn insert_item(connection: &Connection,
    name: String, description: String, path: String,
    color_id: i64, is_directory: bool) -> Result<Item, Error> {

    let is_directory_uint: u8 = match is_directory {
        true => 1,
        false => 0
    };

    let cp_name: String = name.clone();
    let cp_path: String = path.clone();
    let cp_description: String = description.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO item (name, description, path, color_id, is_directory) VALUES (?1, ?2, ?3, ?4, ?5) RETURNING item_id",
        (name, description, path, color_id, is_directory_uint),
        |row| row.get(0)
    )?;

    let item: Item = Item {
        id,
        name: cp_name,
        description: cp_description,
        path: cp_path,
        color_id,
        is_directory,
        tags_id: None,
        categories_id: None
    };

    Ok(item)
}

pub fn insert_tag(connection: &Connection, name: String, description: String, color_id: i64) -> Result<Tag, Error> {
    let cp_name: String = name.clone();
    let cp_description: String = description.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO tag (tag_name, tag_description, color_id) VALUES (?1, ?2, ?3) RETURNING tag_id",
        (name, color_id),
        |row| row.get(0)
    )?;

    let tag: Tag = Tag {
        id,
        name: cp_name,
        description: cp_description,
        color_id,
        linked_tags_id: None,
        categories_id: None
    };

    Ok(tag)
}

pub fn insert_category(connection: &Connection, name: String, description: String, color_id: i64) -> Result<Category, Error> {
    let cp_name: String = name.clone();
    let cp_description: String = description.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO category (category_name, category_description, color_id) VALUES (?1, ?2) RETURNING category_id",
        (name, color_id),
        |row| row.get(0)
    )?;

    let category: Category = Category { 
        id,
        name: cp_name,
        description: cp_description,
        color_id,
    };

    Ok(category)
}

pub fn insert_color(connection: &Connection, name: String, description: String, red: u8, green: u8, blue: u8, alpha: u8) -> Result<Color, Error> {
    let cp_name: String = name.clone();
    let cp_description: String = description.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO color (color_name, color_description, red, green, blue, alpha) VALUES (?1, ?2, ?3, ?4, ?5, ?6) RETURNING color_id",
        (name, description, red, green, blue, alpha),
        |row| row.get(0)
    )?;

    let color: Color = Color {
        id,
        name: cp_name,
        description: cp_description,
        red,
        green,
        blue,
        alpha
    };

    Ok(color)
}

pub fn insert_database(connection: &Connection, name: &str, description: &str, location: &str, color_id: i64, is_in_cloud: bool,
    is_ssh: bool, username: Option<String>, server_address: Option<String>, location_private_key: Option<String>,
    password_encrypted: Option<String>) -> Result<FileSearchDatabase, Error> {

    let name_string: String = name.to_string();
    let description_string: String = description.to_string();
    let location_string: String = location.to_string();

    let slice_username: &str = match username {
        Some(ref u) => u,
        None => ""
    };
    let slice_server_address: &str = match server_address {
        Some(ref u) => u,
        None => ""
    };
    let slice_location_private_key: &str = match location_private_key {
        Some(ref u) => u,
        None => ""
    };
    let slice_password_encrypted: &str = match password_encrypted {
        Some(ref u) => u,
        None => ""
    };


    let id: i64 = connection.query_row(
        "INSERT INTO databases (database_name, database_description, location, is_in_cloud,\
        is_ssh, username, server_address, location_private_key, password_encrypted,\
        color_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) RETURNING database_id",
        (name, description, location, is_in_cloud,
         is_ssh, slice_username, slice_server_address, slice_location_private_key, slice_password_encrypted, color_id),
        |row| row.get(0)
    )?;

    let db: FileSearchDatabase = FileSearchDatabase {
        id,
        name: name_string,
        description: description_string,
        location: location_string,
        color_id,
        is_in_cloud,
        is_ssh,
        username,
        server_address,
        location_private_key,
        password_encrypted,
    };

    Ok(db)
}

