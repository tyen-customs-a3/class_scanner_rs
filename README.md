# Class Scanner

A Rust library for parsing, analyzing and processing class configuration files with inheritance and array operations.

## Features

- Lexical analysis with preprocessing support (including file includes)
- Parsing of class definitions with nested classes and properties
- Inheritance resolution to build complete class hierarchies
- Array operations processing (assignment, append, remove)
- High-level API for common operations
- Comprehensive error reporting

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
class_scanner = "0.1.0"
```

## Basic Usage

```rust
use class_scanner::{ClassScanner, Error};

fn main() -> Result<(), Error> {
    // Create a new scanner
    let scanner = ClassScanner::new();
    
    // Parse a string or file containing class definitions
    let input = r#"
        class Vehicle {
            crew = 1;
            maxSpeed = 120;
        };
        
        class Car: Vehicle {
            crew = 2;
            wheels = 4;
        };
    "#;
    
    let classes = scanner.parse_string(input)?;
    println!("Found {} top-level classes", classes.len());
    
    // Process inheritance to get the complete class with inherited properties
    let processed_car = scanner.process_inheritance(classes, "Car")?;
    println!("Car has {} properties after inheritance", processed_car.properties.len());
    
    Ok(())
}
```

## File Processing

To process class configuration files with includes:

```rust
use class_scanner::{ClassScanner, Error};
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    // Initialize the scanner with a base path for resolving includes
    let base_dir = PathBuf::from("path/to/base/directory");
    let scanner = ClassScanner::new().with_base_path(base_dir);
    
    // Parse a file and process it completely
    let config_path = PathBuf::from("path/to/config.cpp");
    let processed_class = scanner.process_file(config_path, "TargetClassName")?;
    
    // Now you have the fully processed class with inheritance and array operations applied
    println!("Class name: {}", processed_class.name);
    println!("Parent class: {:?}", processed_class.parent);
    println!("Properties: {}", processed_class.properties.len());
    
    Ok(())
}
```

## Working with Arrays

Array operations are a special feature of this parser:

```rust
use class_scanner::{ClassScanner, Error};

fn main() -> Result<(), Error> {
    let scanner = ClassScanner::new();
    
    let input = r#"
        class Weapons {
            rifles[] = {"M4", "AK47", "SCAR"};
            pistols[] = {"Glock", "M1911"};
        };
        
        class WeaponsAddOn: Weapons {
            rifles[] += {"M16"};        // Append to the array
            pistols[] -= {"M1911"};     // Remove from the array
            shotguns[] = {"Remington"}; // Create a new array
        };
    "#;
    
    // Parse and process the class
    let classes = scanner.parse_string(input)?;
    let mut processed_class = scanner.process_inheritance(classes, "WeaponsAddOn")?;
    scanner.process_arrays(&mut processed_class)?;
    
    // Get the resulting arrays
    if let Some(rifles) = processed_class.get_array("rifles") {
        println!("Rifles: {:?}", rifles);  // ["M4", "AK47", "SCAR", "M16"]
    }
    
    if let Some(pistols) = processed_class.get_array("pistols") {
        println!("Pistols: {:?}", pistols);  // ["Glock"]
    }
    
    Ok(())
}
```

## Advanced Example

For a more comprehensive example of how to use all features of the library, see the [examples/basic_usage.rs](./examples/basic_usage.rs) file.

## License

This project is licensed under the MIT License.