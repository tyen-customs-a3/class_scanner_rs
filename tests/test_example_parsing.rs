use class_scanner::{
    models::ClassData,
    parser::{ClassParser, ParserConfig},
};
use std::fs;
use std::path::PathBuf;

const LOADOUT_FILE: &str = "tests/data/example_loadout.hpp";
const MISSION_FILE: &str = "tests/data/example_mission.sqm";

#[test]
fn test_parse_example_loadout() {
    let config = ParserConfig {
        max_depth: 32,
        allow_empty_blocks: true,
        case_sensitive: true,
    };
    let mut parser = ClassParser::with_config(config);
    
    // Read the test file
    let content = fs::read_to_string(LOADOUT_FILE).unwrap();
    parser.set_current_file(PathBuf::from(LOADOUT_FILE));
    
    // Parse hierarchically
    let classes = parser.parse_hierarchical(&content).unwrap();
    
    // Test base class
    let base_man = classes.iter().find(|c| c.name == "baseMan").unwrap();
    assert!(base_man.properties.get("linkedItems[]").is_some());
    let linked_items = base_man.properties["linkedItems[]"].array_values().unwrap();
    assert_eq!(linked_items.len(), 3);
    assert!(linked_items.contains(&"ItemMap".to_string()));

    // Test rifleman class
    let rm = classes.iter().find(|c| c.name == "rm").unwrap();
    assert_eq!(rm.parent, "baseMan");
    assert!(rm.properties.get("displayName").is_some());
    assert_eq!(rm.properties["displayName"].as_string().unwrap(), "Rifleman");

    // Check arrays in rifleman class
    let uniform = rm.properties["uniform[]"].array_values().unwrap();
    assert!(uniform.iter().any(|u| u.contains("aegis_guerilla_garb_m81")));
    
    let items = rm.properties["items[]"].array_values().unwrap();
    assert!(items.iter().any(|i| i.contains("ACRE_PRC343")));

    // Test automatic rifleman class
    let ar = classes.iter().find(|c| c.name == "ar").unwrap();
    assert_eq!(ar.parent, "rm");
    assert!(ar.properties["magazines[]"].array_values().unwrap().iter().any(|i| i.contains("sps_200Rnd_556x45")));
}

#[test]
fn test_parse_example_mission() {
    let config = ParserConfig {
        max_depth: 32,
        allow_empty_blocks: true,
        case_sensitive: true,
    };
    let mut parser = ClassParser::with_config(config);
    
    // Read the test file
    let content = fs::read_to_string(MISSION_FILE).unwrap();
    parser.set_current_file(PathBuf::from(MISSION_FILE));
    
    // Parse hierarchically
    let classes = parser.parse_hierarchical(&content).unwrap();
    
    // Test mission class
    let mission = classes.iter().find(|c| c.name == "Mission").unwrap();
    assert!(mission.nested_classes.iter().any(|c| c.name == "Entities"));

    // Test entities
    let entities = mission.nested_classes.iter().find(|c| c.name == "Entities").unwrap();
    
    // Find the squad leader group (should be in Item1)
    let squad_leader_group = entities.nested_classes.iter()
        .find(|c| c.name == "Item1" && 
              c.nested_classes.iter().any(|nc| nc.name == "Entities"))
        .unwrap();
    
    // Navigate to squad leader entity
    let sl_entities = squad_leader_group.nested_classes.iter()
        .find(|c| c.name == "Entities")
        .unwrap();
        
    let squad_leader = sl_entities.nested_classes.iter()
        .find(|c| c.name == "Item0")
        .unwrap();

    // Check attributes
    let attributes = squad_leader.nested_classes.iter()
        .find(|c| c.name == "Attributes")
        .unwrap();
        
    assert_eq!(attributes.properties["name"].as_string().unwrap(), "B_A_SL");
    assert_eq!(attributes.properties["description"].as_string().unwrap(), "Squad Leader@Alpha Squad");

    // Check custom attributes
    let custom_attrs = squad_leader.nested_classes.iter()
        .find(|c| c.name == "CustomAttributes")
        .unwrap();

    // Find the TMF_assignGear_full attribute (Attribute6)
    let gear_attr = custom_attrs.nested_classes.iter()
        .find(|attr| attr.name == "Attribute6")
        .unwrap();

    assert_eq!(gear_attr.properties["property"].as_string().unwrap(), "TMF_assignGear_full");
}