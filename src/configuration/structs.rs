use serde::Deserialize;
use std::{fs, path::Path};


#[derive(Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Configuration {
    pub title: String,
    pub database: DatabaseConfiguration
}

#[derive(Deserialize, Clone, PartialEq, Debug, Default)]
pub struct DatabaseConfiguration {
    pub location: String,
    pub creation_transaction: String,
    pub creation_transaction_management: String,
}


pub trait ConfigurationManagerTrait {
    fn read(path: String) -> Option<Configuration>;
}

impl ConfigurationManagerTrait for Configuration {
    fn read(path: String) -> Option<Configuration> {
        
        let file_content: String = match fs::read(path) {
            Ok(v) => { match String::from_utf8(v) {
                Ok(str) => str,
                Err(_) => { return None; }
            }},
            Err(_) => { return None; }
        };

        let configuration = match toml::from_str(&file_content) {
            Ok(t) => t,
            Err(_) => { return None; }
        };

        Some(configuration)
    }
}