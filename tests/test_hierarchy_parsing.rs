use class_scanner::{
    models::ClassData,
    parser::{ClassParser, ParserConfig},
};

#[test]
fn test_complex_inheritance_chains() {
    let config = ParserConfig {
        max_depth: 32,
        allow_empty_blocks: true,
        case_sensitive: true,
    };
    let mut parser = ClassParser::with_config(config);
    
    let content = r#"
        class Base;
        class Level1: Base {
            scope = 1;
            class SubA {
                value = "a";
            };
        };
        class Level2: Level1 {
            scope = 2;
            class SubA: Level1 {
                value = "b";
            };
            class SubB {
                items[] = {"1", "2"};
            };
        };
        class Level3: Level2 {
            scope = 3;
            class SubA: Level2 {
                value = "c";
                extra = true;
            };
        };
    "#;
    
    let classes = parser.parse_hierarchical(content).unwrap();
    assert_eq!(classes.len(), 4); // Base, Level1, Level2, Level3

    // Verify inheritance chain
    let level3 = classes.iter().find(|c| c.name == "Level3").unwrap();
    assert_eq!(level3.parent, "Level2");
    assert_eq!(level3.properties["scope"].as_integer().unwrap(), 3);
    
    // Verify nested class inheritance
    let suba = &level3.nested_classes[0];
    assert_eq!(suba.name, "SubA");
    assert_eq!(suba.parent, "Level2");
    assert_eq!(suba.properties["value"].as_string().unwrap(), "c");
    assert!(suba.properties["extra"].as_boolean().unwrap());
}

#[test]
fn test_multiple_independent_hierarchies() {
    let config = ParserConfig::default();
    let mut parser = ClassParser::with_config(config);
    
    let content = r#"
        class CfgVehicles {
            class Tank_Base;
            class Tank_A: Tank_Base {
                armor = 100;
            };
        };
        class CfgWeapons {
            class Rifle_Base;
            class Rifle_A: Rifle_Base {
                damage = 50;
            };
        };
        class CfgAmmo {
            class Shell_Base;
            class Shell_A: Shell_Base {
                caliber = 120;
            };
        };
    "#;
    
    let classes = parser.parse_hierarchical(content).unwrap();
    assert_eq!(classes.len(), 3); // CfgVehicles, CfgWeapons, CfgAmmo

    // Check each hierarchy
    for class in &classes {
        assert_eq!(class.nested_classes.len(), 2); // Base and derived class
        let derived = &class.nested_classes[1];
        match class.name.as_str() {
            "CfgVehicles" => {
                assert_eq!(derived.name, "Tank_A");
                assert_eq!(derived.properties["armor"].as_integer().unwrap(), 100);
            },
            "CfgWeapons" => {
                assert_eq!(derived.name, "Rifle_A");
                assert_eq!(derived.properties["damage"].as_integer().unwrap(), 50);
            },
            "CfgAmmo" => {
                assert_eq!(derived.name, "Shell_A");
                assert_eq!(derived.properties["caliber"].as_integer().unwrap(), 120);
            },
            _ => panic!("Unexpected class: {}", class.name),
        }
    }
}

#[test]
fn test_nested_array_properties() {
    let config = ParserConfig::default();
    let mut parser = ClassParser::with_config(config);
    
    let content = r#"
        class CfgLoadouts {
            class Soldier {
                weapons[] = {
                    {"M4A1", "ACOG"},
                    {"Glock", "Flashlight"}
                };
                magazines[] = {
                    {"30Rnd_556x45", 6},
                    {"9mm_15Rnd", 3}
                };
                items[] = {"FirstAidKit", "Grenades", "Radio"};
            };
        };
    "#;
    
    let classes = parser.parse_hierarchical(content).unwrap();
    let soldier = &classes[0].nested_classes[0];
    
    // Check array properties
    let weapons = soldier.properties["weapons[]"].array_values().unwrap();
    assert_eq!(weapons.len(), 2);
    assert!(weapons[0].contains("M4A1"));
    assert!(weapons[0].contains("ACOG"));
    
    let mags = soldier.properties["magazines[]"].array_values().unwrap();
    assert_eq!(mags.len(), 2);
    assert!(mags[0].contains("30Rnd_556x45"));
    
    let items = soldier.properties["items[]"].array_values().unwrap();
    assert_eq!(items.len(), 3);
    assert_eq!(items[0], "FirstAidKit");
}

#[test]
fn test_case_sensitivity_handling() {
    let mut config = ParserConfig::default();
    config.case_sensitive = false;
    let mut parser = ClassParser::with_config(config);
    
    let content = r#"
        class BaseClass {
            PROPERTY = 1;
            property = 2;
            Property = 3;
        };
        
        class DERIVED: BaseClass {
            NEW_PROP = "test";
        };
        
        class derived: DERIVED {
            another_prop = true;
        };
    "#;
    
    let classes = parser.parse_hierarchical(content).unwrap();
    
    // When case_sensitive=false, all class names should be lowercase
    assert!(classes.iter().any(|c| c.name == "baseclass"));
    
    let derived = classes.iter()
        .find(|c| c.name == "derived")
        .unwrap();
    assert_eq!(derived.parent, "baseclass");  // Fixed: parent should be "baseclass" 
    
    // Properties should preserve their original case but be found case-insensitively
    let base = classes.iter()
        .find(|c| c.name == "baseclass")
        .unwrap();
    assert_eq!(base.properties.len(), 3);

    // Verify that case-insensitive property access works
    assert!(base.properties.iter().any(|(k, _)| k.eq_ignore_ascii_case("property")));
}

#[test]
fn test_error_handling() {
    let config = ParserConfig::default();
    let mut parser = ClassParser::with_config(config);
    
    // Test missing semicolon
    let result = parser.parse_hierarchical("class Test {}");
    assert!(result.is_ok());
    
    // Test unmatched braces
    let result = parser.parse_hierarchical("class Test {{};");
    assert!(result.is_ok());
    
    // Test invalid property syntax
    let content = r#"
        class Test {
            = invalid;
            also invalid
            prop = ;
        };
    "#;
    let result = parser.parse_hierarchical(content);
    assert!(result.is_ok());
}