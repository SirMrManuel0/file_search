use rusqlite::{Connection, Error, };

use crate::db::structs::{Item, Tag, Category, Color};

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
        id: id,
        name: cp_name,
        description: cp_description,
        path: cp_path,
        color_id: color_id,
        is_directory: is_directory,
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
        id: id,
        name: cp_name,
        description: cp_description,
        color_id: color_id,
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
        id: id,
        name: cp_name,
        description: cp_description,
        color_id: color_id,
    };

    Ok(category)
}

pub fn insert_color(connection: &Connection, name: String, description: String, red: u8, green: u8, blue: u8, alpha: u8) -> Result<Color, Error> {
    let cp_name: String = name.clone();
    let cp_description: String = description.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO color (color_name, color_description, red, green, blue, alpha) VALUES (?1, ?2, ?3, ?4, ?5, ?6) RETURNING color_id",
        (red, green, blue, alpha),
        |row| row.get(0)
    )?;

    let color: Color = Color {
        id: id,
        name: cp_name,
        description: cp_description,
        red: red,
        green: green,
        blue: blue,
        alpha: alpha
    };

    Ok(color)
}
