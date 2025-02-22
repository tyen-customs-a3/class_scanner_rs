use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use log::debug;

use class_scanner_rs::{
    ClassData,
    parser::ClassParser,
    error::Result,
};

mod test_helpers;
use test_helpers::setup_test_logging;

// Test data configuration
const TEST_DATA_ROOT: &str = env!("CARGO_MANIFEST_DIR");

lazy_static::lazy_static! {
    static ref TEST_DATA: HashMap<&'static str, TestData> = {
        let mut m = HashMap::new();
        m.insert("hidden_vest", TestData {
            path: PathBuf::from(TEST_DATA_ROOT)
                .join("tests")
                .join("data")
                .join("@pca_misc")
                .join("addons")
                .join("pca_extra_contents"),
            source_path: PathBuf::from(TEST_DATA_ROOT)
                .join("tests")
                .join("data")
                .join("@pca_misc")
                .join("addons")
                .join("pca_extra_contents")
                .join("x")
                .join("pca_misc")
                .join("addons")
                .join("pca_extra_contents")
                .join("CfgWeapons_hiddenVest.hpp"),
            source: "pca_misc".to_string(),
            expected_classes: {
                let mut classes = HashMap::new();
                classes.insert("pca_vest_invisible", ExpectedClass {
                    parent: "Vest_Camo_Base".to_string(),
                    section: Some("_global".to_string()),
                });
                classes.insert("pca_vest_invisible_kevlar", ExpectedClass {
                    parent: "pca_vest_invisible".to_string(),
                    section: Some("_global".to_string()),
                });
                classes.insert("pca_vest_invisible_plate", ExpectedClass {
                    parent: "pca_vest_invisible".to_string(),
                    section: Some("_global".to_string()),
                });
                classes
            }
        });
        m
    };
}

#[derive(Debug, Clone)]
struct TestData {
    path: PathBuf,
    source_path: PathBuf,
    source: String,
    expected_classes: HashMap<&'static str, ExpectedClass>,
}

#[derive(Debug, Clone)]
struct ExpectedClass {
    parent: String,
    section: Option<String>,
}

fn setup_parser(path: &Path) -> ClassParser {
    debug!("Setting up test parser with path: {:?}", path);
    let mut parser = ClassParser::new();
    parser.set_current_file(path);
    parser
}

fn read_test_config(name: &str) -> Result<String> {
    debug!("Reading test config for: {}", name);
    let path = &TEST_DATA[name].source_path;
    Ok(fs::read_to_string(path)?)
}

#[test]
fn test_vest_config_structure() -> Result<()> {
    setup_test_logging();
    let vest_config = read_test_config("hidden_vest")?;
    let mut parser = setup_parser(&TEST_DATA["hidden_vest"].source_path);
    
    debug!("Parsing vest config structure");
    let result = parser.parse_class_definitions(&vest_config)?;
    assert!(!result.is_empty(), "Result should not be empty");
    debug!("Found {} classes in vest config", result.len());

    Ok(())
}

#[test]
fn test_vest_class_inheritance() -> Result<()> {
    setup_test_logging();
    let vest_config = read_test_config("hidden_vest")?;
    let mut parser = setup_parser(&TEST_DATA["hidden_vest"].source_path);
    
    debug!("Parsing vest class inheritance");
    let result = parser.parse_class_definitions(&vest_config)?;
    
    // Check base vest
    let base_vest = result.get("pca_vest_invisible")
        .expect("Base vest class should exist");
    debug!("Base vest parent: {}", base_vest.parent);
    assert_eq!(base_vest.parent, "Vest_Camo_Base");
    
    // Check plate vest
    let plate_vest = result.get("pca_vest_invisible_plate")
        .expect("Plate vest class should exist");
    debug!("Plate vest parent: {}", plate_vest.parent);
    assert_eq!(plate_vest.parent, "pca_vest_invisible");

    Ok(())
}

#[test]
fn test_vest_class_properties() -> Result<()> {
    setup_test_logging();
    let vest_config = read_test_config("hidden_vest")?;
    let mut parser = setup_parser(&TEST_DATA["hidden_vest"].source_path);
    
    debug!("Parsing vest class properties");
    let result = parser.parse_class_definitions(&vest_config)?;
    
    for (class_name, class_data) in result.iter() {
        debug!("Class '{}' has {} properties", class_name, class_data.properties.len());
    }

    Ok(())
}

#[test]
fn test_vest_addon_name() -> Result<()> {
    setup_test_logging();
    let vest_config = read_test_config("hidden_vest")?;
    let mut parser = setup_parser(&TEST_DATA["hidden_vest"].source_path);
    
    debug!("Testing vest addon name extraction");
    let result = parser.parse_class_definitions(&vest_config)?;
    
    let expected_addon = "@pca_misc";
    for (class_name, class_data) in result.iter() {
        if let Some(ref addon) = class_data.addon {
            debug!("Class '{}' has addon name: {}", class_name, addon);
        }
        assert_eq!(
            class_data.addon.as_deref().expect("Addon name should exist"),
            expected_addon,
            "Class {} has wrong addon name: {:?}",
            class_data.name,
            class_data.addon
        );
    }

    Ok(())
}