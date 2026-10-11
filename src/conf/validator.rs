use std::path::Path;

use crate::conf::structs::{
    Configuration,
    DatabaseConfiguration
};

pub fn validate_confiduration (configuration: &Configuration) -> bool {
    if configuration.title != "File Search Configuration" { return false; }

    if !validate_database_configuration(&configuration.database) { return false; }


    true
}

fn validate_database_configuration (database_configuration: &DatabaseConfiguration) -> bool {
    if !Path::new(&database_configuration.creation_transaction).is_file() { return false; }

    true
}
