use class_scanner::{
    ClassScanner, ClassNode, Error,
    ast::{inheritance_visitor::InheritanceVisitor, array_visitor::ArrayVisitor, AstVisitor},
    utils::init_logging,
};
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    // Initialize logging (optional but helpful for debugging)
    if let Err(e) = init_logging(Some("info")) {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }
    
    println!("Class Scanner Basic Usage Example");
    println!("--------------------------------");
    
    // Example 1: Parse a string directly
    println!("\nExample 1: Parsing from a string");
    let input = r#"
        class Vehicle {
            crew = 1;
            maxSpeed = 120;
            
            class Turret {
                weapons[] = {"gun", "missile"};
                magazines[] = {"ammo_belt", "missile_pod"};
            };
        };
        
        class Car: Vehicle {
            crew = 2;
            wheels = 4;
        };
    "#;
    
    let scanner = ClassScanner::new();
    let class_nodes = scanner.parse_string(input)?;
    
    // Print the parsed classes
    for class in &class_nodes {
        print_class(class, 0);
    }
    
    // Example 2: Working with inheritance
    println!("\nExample 2: Processing inheritance");
    
    // Get our classes from the previous example
    let vehicle = find_class_by_name("Vehicle", &class_nodes).expect("Vehicle class not found");
    let car = find_class_by_name("Car", &class_nodes).expect("Car class not found");
    
    // Create and use the inheritance visitor
    let mut inheritance_visitor = InheritanceVisitor::new();
    
    // Register the classes in the inheritance visitor
    inheritance_visitor.register_class(vehicle.clone());
    inheritance_visitor.register_class(car.clone());
    
    // Process the inheritance for the "Car" class
    let processed_car = inheritance_visitor.process("Car")?;
    
    println!("Processed Car class (with inherited properties):");
    print_class(&processed_car, 0);
    
    // Example 3: Processing arrays
    println!("\nExample 3: Working with arrays");
    
    let input_with_arrays = r#"
        class Weapons {
            rifles[] = {"M4", "AK47", "SCAR"};
            pistols[] = {"Glock", "M1911"};
        };
        
        class WeaponsAddOn: Weapons {
            rifles[] += {"M16"};
            pistols[] += {"P226"};
            shotguns[] = {"Remington", "Mossberg"};
        };
    "#;
    
    // Parse the new input
    let array_classes = scanner.parse_string(input_with_arrays)?;
    
    // Register the classes for inheritance processing
    let mut inheritance_visitor = InheritanceVisitor::new();
    for class in &array_classes {
        inheritance_visitor.register_class(class.clone());
    }
    
    // Get the processed class with inheritance
    let processed_weapons = inheritance_visitor.process("WeaponsAddOn")?;
    
    // Create and use the array visitor to process array operations
    let mut array_visitor = ArrayVisitor::new();
    let mut processed_class = processed_weapons.clone();
    array_visitor.visit_class(&mut processed_class)?;
    
    println!("Processed WeaponsAddOn class (with arrays processed):");
    print_class(&processed_class, 0);
    println!("Rifles array: {:?}", processed_class.get_array("rifles"));
    println!("Pistols array: {:?}", processed_class.get_array("pistols"));
    println!("Shotguns array: {:?}", processed_class.get_array("shotguns"));
    
    // Example 4: Using the parser with a file (commented out since we don't have the file)
    /*
    println!("\nExample 4: Parsing from a file");
    let file_path = PathBuf::from("path/to/your/config.cpp");
    let class_nodes = scanner.with_base_path(file_path.parent().unwrap())
        .parse_file(file_path)?;
    */
    
    Ok(())
}

// Helper function to print a class and its contents recursively
fn print_class(class: &ClassNode, indent: usize) {
    let indent_str = " ".repeat(indent * 4);
    println!("{}class {} {}", indent_str, class.name, if let Some(ref parent) = class.parent { 
        format!(": {}", parent) 
    } else { 
        String::new() 
    });
    println!("{}{{", indent_str);
    
    // Print properties
    for property in class.properties.values() {
        println!("{}    {} = {};", indent_str, property.name, property.raw_value);
    }
    
    // Print nested classes
    for nested_class in &class.nested_classes {
        print_class(nested_class, indent + 1);
    }
    
    println!("{}}};", indent_str);
}

// Helper function to find a class by name in the class nodes
fn find_class_by_name<'a>(name: &str, class_nodes: &'a [ClassNode]) -> Option<&'a ClassNode> {
    for class in class_nodes {
        if class.name == name {
            return Some(class);
        }
        
        // Search in nested classes
        for nested in &class.nested_classes {
            if nested.name == name {
                return Some(nested);
            }
        }
    }
    None
}