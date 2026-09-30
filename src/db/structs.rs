#[derive(Debug)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub is_directory: bool,
    pub tags_id: Option<Vec<i64>>,
    pub categories_id: Option<Vec<i64>>,
}

#[derive(Debug)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub linked_tags_id: Option<Vec<i64>>,
    pub categories_id: Option<Vec<i64>>,
}

#[derive(Debug)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}