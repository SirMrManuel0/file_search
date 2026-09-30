use rusqlite::{Connection, Error, };

use crate::db::structs::{Item, Tag, Category};

pub fn insert_item(connection: &Connection, name: String, path: String, rgb: [u8; 3], is_directory: bool) -> Result<Item, Error> {
    let is_directory_uint: u8 = match is_directory {
        true => 1,
        false => 0
    };

    let cp_name: String = name.clone();
    let cp_path: String = path.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO item (name, path, red, green, blue, is_directory) VALUES (?1, ?2, ?3, ?4, ?5, ?6) RETURNING item_id",
        (name, path, rgb[0], rgb[1], rgb[2], is_directory_uint),
        |row| row.get(0)
    )?;

    let item: Item = Item {
        id: id,
        name: cp_name,
        path: cp_path,
        red: rgb[0],
        green: rgb[1],
        blue: rgb[2],
        is_directory: is_directory,
        tags_id: None,
        categories_id: None
    };

    Ok(item)
}

pub fn insert_tag(connection: &Connection, name: String, rgb: [u8; 3]) -> Result<Tag, Error> {
    let cp_name: String = name.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO tag (tag_name, red, green, blue) VALUES (?1, ?2, ?3, ?4) RETURNING tag_id",
        (name, rgb[0], rgb[1], rgb[2]),
        |row| row.get(0)
    )?;

    let tag: Tag = Tag {
        id: id,
        name: cp_name,
        red: rgb[0],
        green: rgb[1],
        blue: rgb[2],
        linked_tags_id: None,
        categories_id: None
    };

    Ok(tag)
}

pub fn insert_category(connection: &Connection, name: String, rgb: [u8; 3]) -> Result<Category, Error> {
    let cp_name: String = name.clone();

    let id: i64 = connection.query_row(
        "INSERT INTO category (category_name, red, green, blue) VALUES (?1, ?2, ?3, ?4) RETURNING category_id",
        (name, rgb[0], rgb[1], rgb[2]),
        |row| row.get(0)
    )?;

    let category: Category = Category { 
        id: id,
        name: cp_name,
        red: rgb[0],
        green: rgb[1],
        blue: rgb[2],
    };

    Ok(category)
}