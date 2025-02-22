use std::collections::HashMap;
use std::path::PathBuf;
use log::{debug, info, warn};

use crate::models::{Error, Result};
use crate::models::ClassData;
use crate::parser::property::PropertyParser;
use crate::parser::ParserConfig;
use crate::parser::block::{BlockHandler, BlockCleaner};
use super::{ClassParsing, ClassParsingContext};

pub struct ClassParser {
    property_parser: PropertyParser,
    block_handler: BlockHandler,
    current_file: Option<PathBuf>,
    current_addon: Option<String>,
    config: ParserConfig,
}

impl ClassParser {
    pub fn new() -> Self {
        debug!("Creating new ClassParser with default configuration");
        let config = ParserConfig::default();
        Self::with_config(config)
    }

    pub fn with_config(config: ParserConfig) -> Self {
        debug!("Creating new ClassParser with custom configuration: {:?}", &config);
        let block_handler = BlockHandler::new(config.clone());
        Self {
            property_parser: PropertyParser::new(),
            block_handler,
            current_file: None,
            current_addon: None,
            config,
        }
    }

    pub fn set_current_file(&mut self, file: impl Into<PathBuf>) {
        let file_path = file.into();
        debug!("Setting current file to: {:?}", file_path);
        self.current_file = Some(file_path);
        self.current_addon = super::utils::extract_addon_name(self.current_file.as_ref().unwrap());
        if let Some(ref addon) = self.current_addon {
            debug!("Extracted addon name: {}", addon);
        }
    }

    pub fn parse_class_definitions(&mut self, content: &str) -> Result<HashMap<String, ClassData>> {
        info!("Starting to parse class definitions from content of length {}", content.len());
        let cleaned_content = self.block_handler.clean_code(content);
        let mut classes = HashMap::new();
        
        let mut pos = 0;
        let context = ClassParsingContext {
            current_file: self.current_file.clone(),
            current_addon: self.current_addon.clone(),
            case_sensitive: self.config.case_sensitive,
        };

        let extractor = super::class_extractor::ClassExtractor::new(
            &self.block_handler,
            &self.property_parser,
            &self.config
        );

        while pos < cleaned_content.len() {
            match extractor.parse_class(&cleaned_content[pos..], &context)? {
                Some((class_data, consumed)) => {
                    debug!("Parsed class: {} at position {}", class_data.name, pos);
                    classes.insert(class_data.name.clone(), class_data);
                    pos += consumed;
                }
                None => break,
            }
        }

        info!("Finished parsing {} classes", classes.len());
        Ok(classes)
    }

    pub fn parse_hierarchical(&self, content: &str) -> Result<Vec<ClassData>> {
        info!("Starting hierarchical class parsing");
        let cleaned_content = self.block_handler.clean_code(content);
        
        super::hierarchy::parse_hierarchical_classes(
            &cleaned_content,
            &self.block_handler,
            &self.property_parser,
            &self.current_file,
            &self.current_addon,
            &self.config
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_parser() -> ClassParser {
        ClassParser::with_config(ParserConfig {
            max_depth: 32,
            allow_empty_blocks: false,
            case_sensitive: true,
        })
    }

    #[test]
    fn test_parse_simple_class() {
        let mut parser = create_parser();
        let content = r#"
            class SimpleClass {
                value = 123;
                text = "test";
            };
        "#;
        let classes = parser.parse_class_definitions(content).unwrap();
        assert_eq!(classes.len(), 1);
        let class = classes.get("SimpleClass").unwrap();
        assert_eq!(class.name, "SimpleClass");
        assert!(class.properties.contains_key("value"));
        assert!(class.properties.contains_key("text"));
    }

    #[test]
    fn test_parse_class_with_inheritance() {
        let mut parser = create_parser();
        let content = r#"
            class BaseClass {};
            class DerivedClass: BaseClass {
                value = 123;
            };
        "#;
        let classes = parser.parse_class_definitions(content).unwrap();
        assert_eq!(classes.len(), 2);
        let derived = classes.get("DerivedClass").unwrap();
        assert_eq!(derived.parent, "BaseClass");
    }

    #[test]
    fn test_parse_cfg_patches() {
        let parser = create_parser();
        let content = r#"
            class CfgPatches {
                class TC_MIRROR {
                    units[] = {"TC_B_Mirror_1"};
                    weapons[] = {"TC_U_Mirror_1"};
                    requiredVersion = 0.1;
                    requiredAddons[] = {"A3_Characters_F"};
                };
            };
        "#;
        let classes = parser.parse_hierarchical(content).unwrap();
        assert_eq!(classes.len(), 1);
        let cfg_patches = &classes[0];
        assert_eq!(cfg_patches.name, "CfgPatches");
        assert_eq!(cfg_patches.nested_classes.len(), 1);
        
        let tc_mirror = &cfg_patches.nested_classes[0];
        assert_eq!(tc_mirror.name, "TC_MIRROR");
        assert!(tc_mirror.properties.contains_key("units[]"));
        assert!(tc_mirror.properties.contains_key("weapons[]"));
    }

    #[test]
    fn test_parse_cfg_weapons() {
        let parser = create_parser();
        let content = r#"
            class CfgWeapons {
                class UniformItem;
                class Uniform_Base;
                class TC_U_Mirror_Base: Uniform_Base {
                    author = "Tyen";
                    scope = 0;
                    class ItemInfo: UniformItem {
                        uniformClass = "TC_B_Mirror_Base";
                        mass = 40;
                    };
                };
            };
        "#;
        let classes = parser.parse_hierarchical(content).unwrap();
        assert_eq!(classes.len(), 1);
        let cfg_weapons = &classes[0];
        
        let base_uniform = &cfg_weapons.nested_classes
            .iter()
            .find(|c| c.name == "TC_U_Mirror_Base")
            .unwrap();
        assert_eq!(base_uniform.parent, "Uniform_Base");
        assert_eq!(base_uniform.properties.get("author").unwrap().as_string().unwrap(), "Tyen");
        
        let item_info = &base_uniform.nested_classes[0];
        assert_eq!(item_info.name, "ItemInfo");
        assert_eq!(item_info.parent, "UniformItem");
    }

    #[test]
    fn test_parse_cfg_vehicles() {
        let parser = create_parser();
        let content = r#"
            class CfgVehicles {
                class B_Soldier_base_F;
                class TC_B_Mirror_Base: B_Soldier_base_F {
                    scope = 0;
                    model = "\tc\mirrorform\uniform\mirror.p3d";
                };
                class TC_B_Mirror_1: TC_B_Mirror_Base {
                    scope = 2;
                    hiddenSelections[] = {"hs_shirt"};
                    hiddenSelectionsTextures[] = {"\tc\mirrorform\uniform\black.paa"};
                };
            };
        "#;
        let classes = parser.parse_hierarchical(content).unwrap();
        let cfg_vehicles = &classes[0];
        
        let mirror_1 = cfg_vehicles.nested_classes
            .iter()
            .find(|c| c.name == "TC_B_Mirror_1")
            .unwrap();
        assert_eq!(mirror_1.parent, "TC_B_Mirror_Base");
        assert!(mirror_1.properties.contains_key("hiddenSelections[]"));
        assert!(mirror_1.properties.contains_key("hiddenSelectionsTextures[]"));
    }

    #[test]
    fn test_addon_name_tracking() {
        let mut parser = create_parser();
        parser.set_current_file(PathBuf::from(r"D:\addons\@tc_mirrorform\addons\mirrorform\config.cpp"));
        let content = "class TestClass {};";
        let classes = parser.parse_class_definitions(content).unwrap();
        let test_class = classes.get("TestClass").unwrap();
        assert_eq!(test_class.addon.as_ref().unwrap(), "@tc_mirrorform");
    }

    #[test]
    fn test_case_sensitivity() {
        let mut parser = ClassParser::with_config(ParserConfig {
            max_depth: 32,
            allow_empty_blocks: false,
            case_sensitive: false,
        });
        let content = r#"
            class CfgWeapons {
                class UPPER_CASE {};
                class lower_case {};
                class MixedCase {};
            };
        "#;
        let classes = parser.parse_class_definitions(content).unwrap();
        assert!(classes.contains_key("cfgweapons"));
        assert!(classes.contains_key("upper_case"));
        assert!(classes.contains_key("lower_case"));
        assert!(classes.contains_key("mixedcase"));
    }
}