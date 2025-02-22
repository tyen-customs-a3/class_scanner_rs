use class_scanner_rs::{
    models::ClassData,
    parser::{ClassParser, ParserConfig},
};
use std::fs;
use std::path::PathBuf;

const TEST_FILE: &str = "tests/data/@tc_mirrorform/addons/mirrorform/tc/mirrorform/config.cpp";

#[test]
fn test_parse_mirrorform_config() {
    let config = ParserConfig {
        max_depth: 32,
        allow_empty_blocks: true,
        case_sensitive: true,
    };
    let mut parser = ClassParser::with_config(config);
    
    // Read the test file
    let content = fs::read_to_string(TEST_FILE).unwrap();
    parser.set_current_file(PathBuf::from(TEST_FILE));
    
    // Parse hierarchically
    let classes = parser.parse_hierarchical(&content).unwrap();
    
    // Test CfgPatches
    let cfg_patches = classes.iter().find(|c| c.name == "CfgPatches").unwrap();
    let tc_mirror = &cfg_patches.nested_classes[0];
    assert_eq!(tc_mirror.name, "TC_MIRROR");
    
    // Verify units and weapons arrays
    let units = tc_mirror.properties.get("units[]").unwrap().array_values().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0], "TC_B_Mirror_1");
    
    let weapons = tc_mirror.properties.get("weapons[]").unwrap().array_values().unwrap();
    assert_eq!(weapons.len(), 1);
    assert_eq!(weapons[0], "TC_U_Mirror_1");
    
    // Test CfgWeapons
    let cfg_weapons = classes.iter().find(|c| c.name == "CfgWeapons").unwrap();
    
    // Test base uniform
    let base_uniform = cfg_weapons.nested_classes.iter()
        .find(|c| c.name == "TC_U_Mirror_Base")
        .unwrap();
    assert_eq!(base_uniform.parent, "Uniform_Base");
    assert_eq!(base_uniform.properties.get("author").unwrap().as_string().unwrap(), "Tyen");
    assert_eq!(base_uniform.properties.get("scope").unwrap().as_integer().unwrap(), 0);
    
    // Test ItemInfo in base uniform
    let item_info = &base_uniform.nested_classes[0];
    assert_eq!(item_info.name, "ItemInfo");
    assert_eq!(item_info.parent, "UniformItem");
    assert_eq!(item_info.properties.get("uniformClass").unwrap().as_string().unwrap(), "TC_B_Mirror_Base");
    
    // Test derived uniform
    let uniform_1 = cfg_weapons.nested_classes.iter()
        .find(|c| c.name == "TC_U_Mirror_1")
        .unwrap();
    assert_eq!(uniform_1.parent, "TC_U_Mirror_Base");
    assert_eq!(uniform_1.properties.get("scope").unwrap().as_integer().unwrap(), 2);
    
    // Test CfgVehicles
    let cfg_vehicles = classes.iter().find(|c| c.name == "CfgVehicles").unwrap();
    
    // Test base vehicle
    let base_vehicle = cfg_vehicles.nested_classes.iter()
        .find(|c| c.name == "TC_B_Mirror_Base")
        .unwrap();
    assert_eq!(base_vehicle.parent, "B_Soldier_base_F");
    assert_eq!(base_vehicle.properties.get("scope").unwrap().as_integer().unwrap(), 0);
    assert_eq!(base_vehicle.properties.get("model").unwrap().as_string().unwrap(), r"\tc\mirrorform\uniform\mirror.p3d");
    
    // Test derived vehicle
    let vehicle_1 = cfg_vehicles.nested_classes.iter()
        .find(|c| c.name == "TC_B_Mirror_1")
        .unwrap();
    assert_eq!(vehicle_1.parent, "TC_B_Mirror_Base");
    assert_eq!(vehicle_1.properties.get("scope").unwrap().as_integer().unwrap(), 2);
    
    // Test arrays in derived vehicle
    let hidden_selections = vehicle_1.properties.get("hiddenSelections[]").unwrap().array_values().unwrap();
    assert_eq!(hidden_selections.len(), 1);
    assert_eq!(hidden_selections[0], "hs_shirt");
    
    let hidden_textures = vehicle_1.properties.get("hiddenSelectionsTextures[]").unwrap().array_values().unwrap();
    assert_eq!(hidden_textures.len(), 1);
    assert_eq!(hidden_textures[0], r"\tc\mirrorform\uniform\black.paa");
}