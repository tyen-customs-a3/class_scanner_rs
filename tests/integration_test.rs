use class_scanner::{
    error::Error,
    lexer::{Tokenizer, Preprocessor},
    parser::Parser,
    ast::{inheritance_visitor::InheritanceVisitor, array_visitor::ArrayVisitor, ClassNode, AstVisitor},
    utils::init_logging,
};
use std::path::PathBuf;

// Helper function to get the test data directory path
fn get_test_data_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("data");
    path
}

fn init_test_logging() {
    init_logging(Some("debug")).ok(); // It's ok if this fails on subsequent calls
}

#[test]
fn test_parse_rhs_headband() -> Result<(), Error> {
    init_test_logging();
    let data_dir = get_test_data_dir();
    let config_path = data_dir.join("@tc_rhs_headband").join("config.cpp");

    // Step 1: Preprocess the file
    let mut preprocessor = Preprocessor::new(&data_dir);
    let content = preprocessor.process_file(&config_path)?;
    println!("Preprocessed content:\n{}", content);

    // Step 2: Tokenize
    let mut tokenizer = Tokenizer::with_file_path(&content, &config_path);
    let tokens = tokenizer.tokenize()?;

    // Step 3: Parse
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse()?;

    // Debug AST structure
    println!("AST structure:");
    for class in &ast.nested_classes {
        println!("Top-level class: {}", class.name);
        for child in &class.nested_classes {
            println!("  Child class: {}", child.name);
        }
    }

    // Step 4: Process visitors
    let mut inheritance_visitor = InheritanceVisitor::new();
    inheritance_visitor.visit_class(&mut ast)?;
    
    let mut array_visitor = ArrayVisitor::new();
    array_visitor.visit_class(&mut ast)?;

    // Step 5: Validate structure
    // First ensure CfgWeapons exists in root classes
    let cfg_weapons = ast.nested_classes.iter()
        .find(|c| c.name == "CfgWeapons")
        .expect("CfgWeapons class not found");
    println!("Found CfgWeapons with {} nested classes", cfg_weapons.nested_classes.len());

    // Find tc_rhs_headband in CfgWeapons nested classes
    let headband = cfg_weapons.nested_classes.iter()
        .find(|c| c.name == "tc_rhs_headband")
        .expect("tc_rhs_headband class not found");
    println!("Found tc_rhs_headband with parent {:?}", headband.parent);

    // Validate inheritance and arrays
    assert_eq!(headband.parent, Some("rhs_headband".to_string()));
    assert!(headband.properties.values().any(|p| p.name == "hiddenSelectionsTextures"));

    Ok(())
}

#[test]
fn test_parse_mirrorform() -> Result<(), Error> {
    init_test_logging();
    let data_dir = get_test_data_dir();
    let config_path = data_dir.join("@tc_mirrorform").join("config.cpp");

    // Step 1: Preprocess the file
    let mut preprocessor = Preprocessor::new(&data_dir);
    let content = preprocessor.process_file(&config_path)?;

    // Step 2: Tokenize
    let mut tokenizer = Tokenizer::with_file_path(&content, &config_path);
    let tokens = tokenizer.tokenize()?;

    // Step 3: Parse
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse()?;

    // Step 4: Process visitors
    let mut inheritance_visitor = InheritanceVisitor::new();
    inheritance_visitor.visit_class(&mut ast)?;
    
    let mut array_visitor = ArrayVisitor::new();
    array_visitor.visit_class(&mut ast)?;

    // Step 5: Find and validate classes
    let find_class = |class_name: &str| -> Option<&ClassNode> {
        for top_level in &ast.nested_classes {
            if let Some(found) = top_level.nested_classes.iter().find(|c| c.name == class_name) {
                return Some(found);
            }
        }
        None
    };

    // Check TC_MIRROR in CfgPatches
    let tc_mirror = find_class("TC_MIRROR").expect("TC_MIRROR class not found");
    
    // Validate arrays
    assert!(tc_mirror.properties.values().any(|p| p.name == "units"));
    assert!(tc_mirror.properties.values().any(|p| p.name == "weapons"));
    assert!(tc_mirror.properties.values().any(|p| p.name == "requiredAddons"));

    // Check inheritance chain
    let mirror_base = find_class("TC_B_Mirror_Base").expect("TC_B_Mirror_Base class not found");
    assert_eq!(mirror_base.parent, Some("B_Soldier_base_F".to_string()));

    let mirror_1 = find_class("TC_B_Mirror_1").expect("TC_B_Mirror_1 class not found");
    assert_eq!(mirror_1.parent, Some("TC_B_Mirror_Base".to_string()));

    Ok(())
}

#[test]
fn test_config_file_errors() {
    init_test_logging();
    let data_dir = get_test_data_dir();

    // Test nonexistent file
    let bad_path = data_dir.join("nonexistent.cpp");
    let mut preprocessor = Preprocessor::new(&data_dir);
    assert!(preprocessor.process_file(&bad_path).is_err());

    // Test malformed config (we'll create a temporary malformed file)
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    
    // Test case 1: Unclosed brace
    let unclosed_path = temp_dir.path().join("unclosed.cpp");
    let unclosed_content = r#"
        class Test {
            value = 123;
            class Nested {
                inner = 456;
            }
            more = 789;
    "#;  // No closing brace but valid token stream
    
    File::create(&unclosed_path)
        .unwrap()
        .write_all(unclosed_content.as_bytes())
        .unwrap();

    let mut preprocessor = Preprocessor::new(temp_dir.path());
    let content = preprocessor.process_file(&unclosed_path).unwrap();
    let mut tokenizer = Tokenizer::with_file_path(&content, &unclosed_path);
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    
    // Parser should fail due to unclosed class
    assert!(parser.parse().is_err());

    // Test case 2: Invalid inheritance
    let invalid_inheritance_path = temp_dir.path().join("invalid_inheritance.cpp");
    let invalid_inheritance_content = r#"
        class Base {
            value = 123;
        };
        class Derived : not_a_class {  // Invalid parent class
            inner = 456;
        };
    "#;

    File::create(&invalid_inheritance_path)
        .unwrap()
        .write_all(invalid_inheritance_content.as_bytes())
        .unwrap();

    let content = preprocessor.process_file(&invalid_inheritance_path).unwrap();
    let mut tokenizer = Tokenizer::with_file_path(&content, &invalid_inheritance_path);
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_ok(), "Parser should accept invalid inheritance - that's handled by the inheritance visitor");

    // Test case 3: Invalid property syntax
    let invalid_property_path = temp_dir.path().join("invalid_property.cpp");
    let invalid_property_content = r#"
        class Test {
            = "missing property name";  // Invalid property syntax
        };
    "#;

    File::create(&invalid_property_path)
        .unwrap()
        .write_all(invalid_property_content.as_bytes())
        .unwrap();

    let content = preprocessor.process_file(&invalid_property_path).unwrap();
    let mut tokenizer = Tokenizer::with_file_path(&content, &invalid_property_path);
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    
    // Parser should fail due to invalid property syntax
    assert!(parser.parse().is_err());
}

#[test]
fn test_parse_pca_misc() -> Result<(), Error> {
    init_test_logging();
    let data_dir = get_test_data_dir();
    let config_path = data_dir.join("@pca_misc").join("CfgWeapons.hpp");

    // Preprocess and parse 
    let mut preprocessor = Preprocessor::new(&data_dir);
    let content = preprocessor.process_file(&config_path)?;
    let mut tokenizer = Tokenizer::with_file_path(&content, &config_path);
    let tokens = tokenizer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse()?;

    // Process visitors
    let mut inheritance_visitor = InheritanceVisitor::new();
    inheritance_visitor.visit_class(&mut ast)?;
    let mut array_visitor = ArrayVisitor::new();
    array_visitor.visit_class(&mut ast)?;

    // Find CfgWeapons
    let cfg_weapons = ast.nested_classes.iter()
        .find(|c| c.name == "CfgWeapons")
        .expect("CfgWeapons class not found");

    // Helper function to find classes in the AST
    fn find_class<'a>(class_name: &str, ast: &'a ClassNode) -> Option<&'a ClassNode> {
        if ast.name == class_name {
            return Some(ast);
        }
        for class in &ast.nested_classes {
            if class.name == class_name {
                return Some(class);
            }
            // Also search nested classes recursively
            for nested in &class.nested_classes {
                if let Some(found) = find_class(class_name, nested) {
                    return Some(found);
                }
            }
        }
        None
    }

    // Test hidden vest inheritance chain
    let vest_base = find_class("pca_vest_invisible", cfg_weapons)
        .expect("pca_vest_invisible class not found");
    assert_eq!(vest_base.parent, Some("Vest_Camo_Base".to_string()));
    
    let vest_kevlar = find_class("pca_vest_invisible_kevlar", cfg_weapons)
        .expect("pca_vest_invisible_kevlar class not found");
    assert_eq!(vest_kevlar.parent, Some("pca_vest_invisible".to_string()));

    let vest_plate = find_class("pca_vest_invisible_plate", cfg_weapons)
        .expect("pca_vest_invisible_plate class not found");
    assert_eq!(vest_plate.parent, Some("pca_vest_invisible".to_string()));

    // Test MICH helmet classes and their texture arrays
    let mich_desert = find_class("pca_mich_norotos_desert", cfg_weapons)
        .expect("pca_mich_norotos_desert class not found");
    assert_eq!(mich_desert.parent, Some("rhsusf_mich_bare_norotos_tan".to_string()));
    
    // Verify texture array property exists and has correct length
    let textures = mich_desert.properties.values()
        .find(|p| p.name == "hiddenSelectionsTextures")
        .expect("hiddenSelectionsTextures not found");
    assert_eq!(textures.array_values.len(), 3); // Should have 3 texture paths

    // Test legacy uniform inheritance
    let uniform = find_class("rhs_uniform_m88_patchless", cfg_weapons)
        .expect("rhs_uniform_m88_patchless class not found");
    assert_eq!(uniform.parent, Some("rhs_uniform_flora".to_string()));

    Ok(())
}