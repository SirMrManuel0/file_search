use rusqlite::{Connection, Error, };

use crate::db::structs::{Item, Tag, Category};

pub fn link_item_to_tag(connection: &Connection, item: &mut Item, tag: &Tag) -> Result<(), Error> {
    connection.execute(
        "INSERT INTO item_tag_index (item_id, tag_id) VALUES (?1, ?2)",
        (item.id, tag.id)
    )?;

    item.tags_id
        .get_or_insert_with(Vec::new)
        .push(tag.id);

    Ok(())
}

pub fn link_item_to_category(connection: &Connection, item: &mut Item, category: &Category) -> Result<(), Error> {
    connection.execute(
        "INSERT INTO item_category_index (item_tag, category_id) VALUES (?1, ?2)",
        (item.id, category.id)
    )?;

    item.categories_id
        .get_or_insert_with(Vec::new)
        .push(category.id);

    Ok(())
}

pub fn link_tags(connection: &Connection, tag: &mut Tag, tag_to_link: &mut Tag) -> Result<(), Error> {
    connection.execute(
        "INSERT INTO tag_linked_tag_index (tag_id, linked_tag_id) VALUES (?1, ?2)",
        (tag.id, tag_to_link.id)
    )?;

    connection.execute(
        "INSERT INTO tag_linked_tag_index (tag_id, linked_tag_id) VALUES (?1, ?2)",
        (tag_to_link.id, tag.id)
    )?;

    tag.linked_tags_id
        .get_or_insert_with(Vec::new)
        .push(tag_to_link.id);

    tag_to_link
        .linked_tags_id
        .get_or_insert_with(Vec::new)
        .push(tag.id);


    Ok(())
}

pub fn link_tag_to_category(connection: &Connection, tag: &mut Tag, category: &Category) -> Result<(), Error> {
    connection.execute(
        "INSERT INTO tag_category_index (tag_id, category_id) VALUES (?1, ?2)",
        (tag.id, category.id)
    )?;

    tag.linked_tags_id
        .get_or_insert_with(Vec::new)
        .push(category.id);

    Ok(())
}
