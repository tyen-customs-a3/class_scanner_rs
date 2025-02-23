use crate::parser::property::PropertyParser;

#[test]
fn test_parse_simple_properties() {
    let parser = PropertyParser::new();
    let content = r#"
        value = 123;
        text = "test";
        flag = true;
        float = 1.234;
        path = \some\path\file.paa;
    "#;
    
    let properties = parser.parse_block_properties(content).unwrap();
    assert_eq!(properties.len(), 5);
    assert_eq!(properties["value"].as_integer().unwrap(), 123);
    assert_eq!(properties["text"].as_string().unwrap(), "test");
    assert_eq!(properties["flag"].as_boolean().unwrap(), true);
    assert_eq!(properties["float"].as_number().unwrap(), 1.234);
    assert_eq!(properties["path"].as_string().unwrap(), r"\some\path\file.paa");
}

#[test]
fn test_parse_array_properties() {
    let parser = PropertyParser::new();
    let content = r#"
        empty[] = {};
        single[] = {"one"};
        items[] = {"one", "two"};
        mixed[] = {1, "two", true};
        nested[] = {{"a", "b"}, {"c", "d"}};
        paths[] = {\path\one.paa, \path\two.paa};
    "#;
    
    let properties = parser.parse_block_properties(content).unwrap();
    assert_eq!(properties["empty[]"].array_values().unwrap().len(), 0);
    assert_eq!(properties["single[]"].array_values().unwrap().len(), 1);
    assert_eq!(properties["items[]"].array_values().unwrap().len(), 2);
    assert_eq!(properties["mixed[]"].array_values().unwrap().len(), 3);
    assert_eq!(properties["nested[]"].array_values().unwrap().len(), 2);
    assert_eq!(properties["paths[]"].array_values().unwrap().len(), 2);
    
    // Verify array contents
    assert_eq!(properties["single[]"].array_values().unwrap()[0], "one");
    assert_eq!(properties["items[]"].array_values().unwrap()[1], "two");
    assert_eq!(properties["paths[]"].array_values().unwrap()[0], r"\path\one.paa");
}

#[test]
fn test_parse_complex_values() {
    let parser = PropertyParser::new();
    let content = r#"
        model = "\tc\mirrorform\uniform\mirror.p3d";
        items[] = {{"item1", 1}, {"item2", 2}};
        config[] = {{"type", "weapon"}, {"slot", "primary"}};
        escaped = "Value with \"quotes\" inside";
        multiline = "Line1" \n "Line2";
    "#;
    
    let properties = parser.parse_block_properties(content).unwrap();
    assert_eq!(properties["model"].as_string().unwrap(), r"\tc\mirrorform\uniform\mirror.p3d");
    assert_eq!(properties["items[]"].array_values().unwrap().len(), 2);
    assert_eq!(properties["config[]"].array_values().unwrap().len(), 2);
    assert!(properties["escaped"].as_string().unwrap().contains("\"quotes\""));
}

#[test]
fn test_parse_edge_cases() {
    let parser = PropertyParser::new();
    let content = r#"
        empty_string = "";
        space_string = " ";
        quoted_number = "123";
        special_chars = _-./\;
        empty_array[] = {};
        spaced_array[] = { "a" , "b" };
    "#;
    
    let properties = parser.parse_block_properties(content).unwrap();
    assert_eq!(properties["empty_string"].as_string().unwrap(), "");
    assert_eq!(properties["space_string"].as_string().unwrap(), " ");
    assert_eq!(properties["quoted_number"].as_string().unwrap(), "123");
    assert_eq!(properties["special_chars"].as_string().unwrap(), "_-./\\");
    assert_eq!(properties["empty_array[]"].array_values().unwrap().len(), 0);
    assert_eq!(properties["spaced_array[]"].array_values().unwrap().len(), 2);
}

#[test]
fn test_invalid_properties() {
    let parser = PropertyParser::new();
    
    // Missing semicolon
    let result = parser.parse_block_properties("value = 123");
    assert!(result.is_ok(), "Should handle missing semicolon gracefully");
    
    // Missing equals
    let result = parser.parse_block_properties("value 123;");
    assert!(result.is_ok(), "Should handle missing equals gracefully");
    
    // Empty property
    let result = parser.parse_block_properties("value=;");
    assert!(result.is_ok(), "Should handle empty value gracefully");
}