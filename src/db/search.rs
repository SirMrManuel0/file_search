use rusqlite::{Connection, Error, };

use crate::db::structs::{Item, Tag, Category};

pub fn search_tag(connection: &Connection, incomplete_name: String) -> Result<Vec<Tag>, Error> {
    let mut statement: rusqlite::Statement<'_> = connection.prepare(
        "SELECT tag_id, tag_name, red, green, blue FROM tag WHERE tag_name LIKE %?1%"
    )?;

    let tag_iter = statement.query_map((incomplete_name, ), |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            red: row.get(2)?,
            green: row.get(3)?,
            blue: row.get(4)?,
            linked_tags_id: None,
            categories_id: None
        })
    })?;

    let mut tag_vec: Vec<Tag> = Vec::new();

    for tag in tag_iter {
        match tag {
            Ok(t) => tag_vec.push(t),
            Err(_) => {}
        }
    }

    Ok(Vec::new())
}