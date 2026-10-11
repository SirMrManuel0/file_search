use rusqlite::{Connection, Error, };

use crate::db::structs::{Item, Tag, Category};

pub fn search_tag_with_incomplete_name(connection: &Connection, incomplete_name: String) -> Result<Vec<Tag>, Error> {
    let mut statement: rusqlite::Statement<'_> = connection.prepare(
        "SELECT tag_id, tag_name, tag_description, color_id FROM tag WHERE tag_name LIKE '%' || ?1 || '%'"
    )?;

    let tag_vec: Vec<Tag> = statement.query_map((incomplete_name, ), |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            color_id: row.get(3)?,
            linked_tags_id: None,
            categories_id: None
        })
    })?.collect::<Result<Vec<Tag>,_>>()?;

    Ok(tag_vec)
}

pub fn search_item_with_incomplete_name(connection: &Connection, incomplete_name: String) -> Result<Vec<Item>, Error> {
    let mut statement: rusqlite::Statement<'_> = connection.prepare(
        "SELECT item_id, name, path, description, color_id, is_directory FROM item WHERE name LIKE '%' || ?1 || '%'"
    )?;

    let item_vec: Vec<Item> = statement.query_map((incomplete_name, ), |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            description: row.get(3)?,
            color_id: row.get(4)?,
            is_directory: row.get(5)?,
            tags_id: None,
            categories_id: None
        })
    })?.collect::<Result<Vec<Item>,_>>()?;

    Ok(item_vec)
}

pub fn search_category_with_incomplete_name(connection: &Connection, incomplete_name: String) -> Result<Vec<Category>, Error> {
    let mut statement = connection.prepare(
        "SELECT category_id, category_name, category_description, color_id FROM category WHERE category_name LIKE '%' || ?1 || '%'"
    )?;

    let categories_vec: Vec<Category> = statement.query_map((incomplete_name,), |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            color_id: row.get(3)?,
        })
    })?.collect::<Result<Vec<Category>, _>>()?;

    Ok(categories_vec)
}

pub fn search_tag_with_incomplete_description(connection: &Connection, incomplete_description: String) -> Result<Vec<Tag>, Error> {
    let mut statement = connection.prepare(
        "SELECT tag_id, tag_name, tag_description, color_id FROM tag WHERE tag_description LIKE '%' || ?1 || '%'"
    )?;

    let tag_vec = statement.query_map((incomplete_description,), |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            color_id: row.get(3)?,
            linked_tags_id: None,
            categories_id: None,
        })
    })?.collect::<Result<Vec<Tag>, _>>()?;

    Ok(tag_vec)
}
