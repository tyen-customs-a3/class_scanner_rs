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
    init_logging(Some("info")).ok(); // It's ok if this fails on subsequent calls
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
fn test_parse_hidden_vest() -> Result<(), Error> {
    init_test_logging();
    let data_dir = get_test_data_dir();
    let config_path = data_dir.join("@pca_misc").join("CfgWeapons_hiddenVest.hpp");

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
        ast.nested_classes.iter()
            .find(|c| c.name == class_name)
    };

    // Validate base invisible vest
    let base_vest = find_class("pca_vest_invisible")
        .expect("pca_vest_invisible class not found");
    assert_eq!(base_vest.parent, Some("Vest_Camo_Base".to_string()));
    
    // Find the ItemInfo class within base_vest
    let base_iteminfo = base_vest.nested_classes.iter()
        .find(|c| c.name == "ItemInfo")
        .expect("ItemInfo class not found in pca_vest_invisible");
    
    // Verify base vest properties
    assert_eq!(base_iteminfo.properties.get("mass").map(|p| p.raw_value.as_str()), Some("20"));

    // Validate kevlar vest
    let kevlar_vest = find_class("pca_vest_invisible_kevlar")
        .expect("pca_vest_invisible_kevlar class not found");
    assert_eq!(kevlar_vest.parent, Some("pca_vest_invisible".to_string()));

    let kevlar_iteminfo = kevlar_vest.nested_classes.iter()
        .find(|c| c.name == "ItemInfo")
        .expect("ItemInfo class not found in pca_vest_invisible_kevlar");
    
    // Verify kevlar vest properties
    assert_eq!(kevlar_iteminfo.properties.get("mass").map(|p| p.raw_value.as_str()), Some("40"));

    // Validate plate vest
    let plate_vest = find_class("pca_vest_invisible_plate")
        .expect("pca_vest_invisible_plate class not found");
    assert_eq!(plate_vest.parent, Some("pca_vest_invisible".to_string()));

    let plate_iteminfo = plate_vest.nested_classes.iter()
        .find(|c| c.name == "ItemInfo")
        .expect("ItemInfo class not found in pca_vest_invisible_plate");
    
    // Verify plate vest properties
    assert_eq!(plate_iteminfo.properties.get("mass").map(|p| p.raw_value.as_str()), Some("80"));

    // Helper function to validate protection info
    let validate_protection_info = |iteminfo: &ClassNode, expected_armor: i32, expected_pass_through: f32| {
        let hitpoints = iteminfo.nested_classes.iter()
            .find(|c| c.name == "HitpointsProtectionInfo")
            .expect("HitpointsProtectionInfo class not found");

        for part in ["Chest", "Diaphragm", "Abdomen", "Pelvis"] {
            let body_part = hitpoints.nested_classes.iter()
                .find(|c| c.name == part)
                .unwrap_or_else(|| panic!("{} class not found", part));
            
            assert_eq!(body_part.properties.get("armor").map(|p| p.raw_value.as_str()), 
                      Some(expected_armor.to_string().as_str()),
                      "Armor value mismatch for {}", part);
            
            assert_eq!(body_part.properties.get("passThrough").map(|p| p.raw_value.as_str()), 
                      Some(expected_pass_through.to_string().as_str()),
                      "PassThrough value mismatch for {}", part);
        }

        // Check Body part which only has passThrough
        let body = hitpoints.nested_classes.iter()
            .find(|c| c.name == "Body")
            .expect("Body class not found");
        assert_eq!(body.properties.get("passThrough").map(|p| p.raw_value.as_str()), 
                  Some(expected_pass_through.to_string().as_str()),
                  "PassThrough value mismatch for Body");
    };

    // Validate base vest protection values
    validate_protection_info(base_iteminfo, 4, 0.5);

    // Validate kevlar vest protection values
    validate_protection_info(kevlar_iteminfo, 12, 0.4);

    // Validate plate vest protection values
    validate_protection_info(plate_iteminfo, 24, 0.2);

    Ok(())
}

#[test]
fn test_parse_3den_config() -> Result<(), Error> {
    // init_test_logging();
    let data_dir = get_test_data_dir();
    let config_path = data_dir.join("a3_Addons_3den_a3_3den_config.cpp");

    // Step 1: Preprocess the file
    let mut preprocessor = Preprocessor::new(&data_dir);
    println!("Starting preprocessing of 3DEN config...");
    let content = match preprocessor.process_file(&config_path) {
        Ok(content) => {
            println!("Successfully preprocessed file. Content length: {}", content.len());
            // Print first 200 chars of content for debugging
            println!("File start:\n{}", &content[..200.min(content.len())]);
            content
        },
        Err(e) => {
            println!("Preprocessing failed: {:?}", e);
            return Err(e);
        }
    };

    // Step 2: Tokenize with extra error context
    println!("Starting tokenization...");
    let mut tokenizer = Tokenizer::with_file_path(&content, &config_path);
    let tokens = match tokenizer.tokenize() {
        Ok(tokens) => {
            println!("Successfully tokenized file. Token count: {}", tokens.len());
            tokens
        },
        Err(e) => {
            println!("Tokenization failed: {:?}", e);
            return Err(e);
        }
    };

    // Step 3: Parse with detailed error checking
    println!("Starting parsing...");
    let mut parser = Parser::new(tokens.clone());
    match parser.parse() {
        Ok(ast) => {
            println!("Successfully parsed file. Root class count: {}", ast.nested_classes.len());
            // ...rest of the function...
            Ok(())
        },
        Err(e) => {
            println!("Parsing failed: {}", e);
            // Print context around the error
            if let Error::ParseError { location, .. } = &e {
                if location.line > 0 {
                    println!("\nContext around error:");
                    let error_line = location.line;
                    let error_col = location.column;
                    
                    // Find tokens around the error location
                    let error_tokens: Vec<_> = tokens.iter()
                        .filter(|t| t.line >= error_line.saturating_sub(2) && t.line <= error_line + 2)
                        .collect();
                    
                    println!("Tokens near line {}:", error_line);
                    for token in error_tokens {
                        println!("  {:?} at line {}, column {}{}", 
                            token.token_type, token.line, token.column,
                            if token.line == error_line && token.column == error_col { " <-- ERROR HERE" } else { "" }
                        );
                    }
                }
            }
            Err(e)
        }
    }
}

