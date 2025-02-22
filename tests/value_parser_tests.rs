use class_scanner_rs::{
    parser::{ParserConfig, ValueParser},
    models::PropertyValueType,
};

#[test]
fn test_string_values() {
    let parser = ValueParser::new(ParserConfig::default());
    
    // Test quoted strings
    let (type_, value) = parser.detect_type(r#""Tyen""#).unwrap();
    assert_eq!(type_, PropertyValueType::String);
    assert_eq!(value, "Tyen");
    
    let (type_, value) = parser.detect_type(r#""Mirrorform""#).unwrap();
    assert_eq!(type_, PropertyValueType::String);
    assert_eq!(value, "Mirrorform");
}

#[test]
fn test_number_values() {
    let parser = ValueParser::new(ParserConfig::default());
    
    // Test scope values
    let (type_, value) = parser.detect_type("0").unwrap();
    assert_eq!(type_, PropertyValueType::Number);
    assert_eq!(value, "0");
    
    let (type_, value) = parser.detect_type("2").unwrap();
    assert_eq!(type_, PropertyValueType::Number);
    assert_eq!(value, "2");
    
    // Test version numbers
    let (type_, value) = parser.detect_type("0.1").unwrap();
    assert_eq!(type_, PropertyValueType::Number);
    assert_eq!(value, "0.1");
    
    // Test mass value
    let (type_, value) = parser.detect_type("40").unwrap();
    assert_eq!(type_, PropertyValueType::Number);
    assert_eq!(value, "40");
}

#[test]
fn test_identifier_values() {
    let parser = ValueParser::new(ParserConfig::default());
    
    // Test class names
    let (type_, value) = parser.detect_type("UniformItem").unwrap();
    assert_eq!(type_, PropertyValueType::Identifier);
    
    let (type_, value) = parser.detect_type("Uniform_Base").unwrap();
    assert_eq!(type_, PropertyValueType::Identifier);
    
    // Test more complex identifiers
    let (type_, value) = parser.detect_type("TC_B_Mirror_Base").unwrap();
    assert_eq!(type_, PropertyValueType::Identifier);
    
    let (type_, value) = parser.detect_type("B_Soldier_base_F").unwrap();
    assert_eq!(type_, PropertyValueType::Identifier);
}

#[test]
fn test_file_paths() {
    let parser = ValueParser::new(ParserConfig::default());
    
    // Test model paths
    let (type_, value) = parser.detect_type(r#""\tc\mirrorform\uniform\mirror.p3d""#).unwrap();
    assert_eq!(type_, PropertyValueType::String);
    assert_eq!(value, r#"\tc\mirrorform\uniform\mirror.p3d"#);
    
    // Test texture paths
    let (type_, value) = parser.detect_type(r#""\tc\mirrorform\uniform\black.paa""#).unwrap();
    assert_eq!(type_, PropertyValueType::String);
    assert_eq!(value, r#"\tc\mirrorform\uniform\black.paa"#);
}

#[test]
fn test_special_values() {
    let parser = ValueParser::new(ParserConfig::default());
    
    // Test the special "-" value often used in uniformModel
    let (type_, value) = parser.detect_type(r#""-""#).unwrap();
    assert_eq!(type_, PropertyValueType::String);
    assert_eq!(value, "-");
}