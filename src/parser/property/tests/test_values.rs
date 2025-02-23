use crate::parser::property::PropertyValue;

#[test]
fn test_parse_string_values() {
    let tests = vec![
        (r#""simple string""#, "simple string"),
        (r#""quoted \"string\"""#, r#"quoted "string""#),
        (r#""path\\to\\file.paa""#, r#"path\to\file.paa"#),
        (r#""multi\nline""#, "multi\nline"),
        (r#""tab\tseparated""#, "tab\tseparated"),
        (r#""mixed\\path/style""#, r#"mixed\path\style"#),
    ];

    for (input, expected) in tests {
        if let PropertyValue::String(value) = PropertyValue::parse(input, true).unwrap() {
            assert_eq!(value, expected);
        } else {
            panic!("Expected String variant for input: {}", input);
        }
    }
}

#[test]
fn test_parse_numeric_values() {
    assert!(matches!(PropertyValue::parse("123", true).unwrap(), PropertyValue::Integer(123)));
    assert!(matches!(PropertyValue::parse("-456", true).unwrap(), PropertyValue::Integer(-456)));
    assert!(matches!(PropertyValue::parse("1.234", true).unwrap(), PropertyValue::Number(1.234)));
    assert!(matches!(PropertyValue::parse("-5.678", true).unwrap(), PropertyValue::Number(-5.678)));
}

#[test]
fn test_parse_boolean_values() {
    assert!(matches!(PropertyValue::parse("true", true).unwrap(), PropertyValue::Boolean(true)));
    assert!(matches!(PropertyValue::parse("false", true).unwrap(), PropertyValue::Boolean(false)));
    assert!(matches!(PropertyValue::parse("TRUE", true).unwrap(), PropertyValue::Boolean(true)));
    assert!(matches!(PropertyValue::parse("FALSE", true).unwrap(), PropertyValue::Boolean(false)));
}

#[test]
fn test_parse_array_values() {
    let tests = vec![
        ("{}", 0),
        (r#"{"single"}"#, 1),
        (r#"{"one", "two"}"#, 2),
        (r#"{"a", "b", "c"}"#, 3),
        (r#"{{"nested", "array"}, {"second", "part"}}"#, 2),
    ];

    for (input, expected_len) in tests {
        if let PropertyValue::Array(values) = PropertyValue::parse(input, true).unwrap() {
            assert_eq!(values.len(), expected_len);
        } else {
            panic!("Expected Array variant for input: {}", input);
        }
    }
}

#[test]
fn test_parse_identifiers() {
    let tests = vec![
        "simple_ident",
        "WITH_CAPS",
        "mixed_Case_123",
        r"path\\to\\file",
        "class.subclass",
    ];

    for input in tests {
        if let PropertyValue::Identifier(value) = PropertyValue::parse(input, true).unwrap() {
            assert_eq!(value, input);
        } else {
            panic!("Expected Identifier variant for input: {}", input);
        }
    }

    // Test case sensitivity
    if let PropertyValue::Identifier(value) = PropertyValue::parse("UPPERCASE", false).unwrap() {
        assert_eq!(value, "uppercase");
    }
}

#[test]
fn test_type_conversions() {
    let string_val = PropertyValue::String("test".to_string());
    let num_val = PropertyValue::Number(1.23);
    let int_val = PropertyValue::Integer(456);
    let bool_val = PropertyValue::Boolean(true);
    let array_val = PropertyValue::Array(vec!["one".to_string(), "two".to_string()]);

    // Test successful conversions
    assert_eq!(String::try_from(&string_val).unwrap(), "test");
    assert_eq!(f64::try_from(&num_val).unwrap(), 1.23);
    assert_eq!(i64::try_from(&int_val).unwrap(), 456);
    assert_eq!(bool::try_from(&bool_val).unwrap(), true);
    assert_eq!(Vec::<String>::try_from(&array_val).unwrap(), vec!["one", "two"]);

    // Test failed conversions
    assert!(String::try_from(&bool_val).is_err());
    assert!(f64::try_from(&string_val).is_err());
    assert!(i64::try_from(&array_val).is_err());
    assert!(bool::try_from(&num_val).is_err());
}

#[test]
fn test_value_accessors() {
    let string_val = PropertyValue::String("test".to_string());
    let num_val = PropertyValue::Number(1.23);
    let int_val = PropertyValue::Integer(456);
    let bool_val = PropertyValue::Boolean(true);
    let array_val = PropertyValue::Array(vec!["one".to_string(), "two".to_string()]);

    // Test optional accessors
    assert_eq!(string_val.as_string().unwrap(), "test");
    assert_eq!(num_val.as_number().unwrap(), 1.23);
    assert_eq!(int_val.as_integer().unwrap(), 456);
    assert_eq!(bool_val.as_boolean().unwrap(), true);
    assert_eq!(array_val.as_array().unwrap(), &["one", "two"]);

    // Test required accessors
    assert_eq!(string_val.require_string().unwrap(), "test");
    assert_eq!(num_val.require_number().unwrap(), 1.23);
    assert_eq!(int_val.require_integer().unwrap(), 456);
    assert_eq!(bool_val.require_boolean().unwrap(), true);
    assert_eq!(array_val.require_array().unwrap(), &["one", "two"]);

    // Test type mismatches
    assert!(string_val.as_number().is_none());
    assert!(num_val.as_boolean().is_none());
    assert!(bool_val.as_array().is_none());
    assert!(array_val.as_string().is_none());

    assert!(string_val.require_number().is_err());
    assert!(num_val.require_boolean().is_err());
    assert!(bool_val.require_array().is_err());
    assert!(array_val.require_string().is_err());
}

#[test]
fn test_array_handling() {
    let empty = PropertyValue::array(vec![]);
    let single = PropertyValue::array(vec!["one".to_string()]);
    let multiple = PropertyValue::array(vec!["one".to_string(), "two".to_string()]);

    assert!(empty.is_array());
    assert!(single.is_array());
    assert!(multiple.is_array());

    assert_eq!(empty.array_values().unwrap().len(), 0);
    assert_eq!(single.array_values().unwrap().len(), 1);
    assert_eq!(multiple.array_values().unwrap().len(), 2);

    assert_eq!(single.array_values().unwrap()[0], "one");
    assert_eq!(multiple.array_values().unwrap()[1], "two");
}

#[test]
fn test_single_value_parsing() {
    // Test automatic type inference in single()
    assert!(matches!(PropertyValue::single("123".to_string()), PropertyValue::Integer(123)));
    assert!(matches!(PropertyValue::single("1.23".to_string()), PropertyValue::Number(1.23)));
    assert!(matches!(PropertyValue::single("true".to_string()), PropertyValue::Boolean(true)));
    assert!(matches!(PropertyValue::single("simple_id".to_string()), PropertyValue::Identifier(_)));
    assert!(matches!(PropertyValue::single("\"quoted\"".to_string()), PropertyValue::String(_)));
}

#[test]
fn test_escape_helpers() {
    let tests = vec![
        ("simple", "simple"),
        (r#"with"quotes"#, r#"with\"quotes"#),
        ("new\nline", r#"new\nline"#),
        (r#"path\to\file"#, r#"path\\to\\file"#),
        ("tab\there", r#"tab\there"#),
    ];

    for (input, expected) in tests {
        assert_eq!(PropertyValue::escape_string(input), expected);
        assert_eq!(PropertyValue::needs_escaping(input), input != expected);
    }
}

#[test]
fn test_to_string_escaped() {
    let tests = vec![
        (PropertyValue::String("simple".to_string()), "simple"),
        (PropertyValue::String("with\"quote".to_string()), r#""with\"quote""#),
        (PropertyValue::String("path\\to\\file".to_string()), r#""path\\to\\file""#),
        (PropertyValue::Array(vec!["a".to_string(), "b".to_string()]), "{a, b}"),
        (PropertyValue::Array(vec![r#"a"b"#.to_string()]), r#"{\"a\"b\"}"#),
    ];

    for (value, expected) in tests {
        assert_eq!(value.to_string_escaped(), expected);
    }
}

#[test]
fn test_path_normalization() {
    let tests = vec![
        (r#"path/to/file"#, r#"path\to\file"#),
        (r#"path\\to\\file"#, r#"path\to\file"#),
        (r#"mixed/style\\path"#, r#"mixed\style\path"#),
        (r#"\\server\share"#, r#"\server\share"#),
    ];

    for (input, expected) in tests {
        if let PropertyValue::String(value) = PropertyValue::parse(input, true).unwrap() {
            assert_eq!(value, expected);
        }
    }
}