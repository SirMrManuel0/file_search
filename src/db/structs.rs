#[derive(Debug, Clone, PartialEq, Default)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub path: String,
    pub color_id: i64,
    pub is_directory: bool,
    pub tags_id: Option<Vec<i64>>,
    pub categories_id: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color_id: i64,
    pub linked_tags_id: Option<Vec<i64>>,
    pub categories_id: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color_id: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Color {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileSearchDatabase {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub location: String,
    pub is_in_cloud: bool,
    pub is_ssh: bool,
    pub username: Option<String>,
    pub password_encrypted: Option<String>,
    pub location_private_key: Option<String>,
    pub server_address: Option<String>,
    pub color_id: i64,
}
