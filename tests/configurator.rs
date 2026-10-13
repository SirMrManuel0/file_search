use file_search::configuration::{Configuration, ConfigurationManagerTrait, DatabaseConfiguration};
use file_search::configuration::validator;

mod common;

#[test]
fn test_reading_configuration() {
    let configuration: Configuration = match Configuration::read("conf.toml".to_string()) {
        Some(conf) => conf,
        None => { panic!("Configuration file not found"); }
    };

    let good_configuration: Configuration = common::get_configuration();

    assert_eq!(configuration, good_configuration);
}

#[test]
fn test_validator_configuration() {
    let good_configuration: Configuration = common::get_configuration();
    assert!(validator::validate_configuration(&good_configuration));
    assert!(validator::validate_database_configuration(&good_configuration.database));
}