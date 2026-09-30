use toml::de::Error;
use std::fs;

pub trait configuration_trait {
    fn read(path: String) -> Option<()>;
    fn generate_structs() -> Option<()>;
}

pub trait configuration_struct_generator_trait {
    fn create_struct<T>() -> Option<T>;
}

pub struct Configuration;

pub impl configuration_trait for Configuration {
    fn read(path: String) -> Option<()> {
        
        let file_content: String = match fs::read(path) {
            Ok(v) => { match String::from_utf8(v) {
                Ok(str) => str,
                Err(_) => { return None; }
            }},
            Err(_) => { return None; }
        }:

        toml::from_str()
        Ok(())
    }

    fn generate_structs() -> Option<()> {
        
    }
}