//! Parser module for handling class definitions and configurations
//! 
//! This module provides the core parsing functionality for class definitions,
//! including support for inheritance, nested classes, and property parsing.

pub mod block;
pub mod class;
pub mod config;
pub mod patterns;
pub mod property;
pub mod tokenizer;
pub mod tokens;

pub use config::*;
pub use property::*;
pub use class::parser::ClassParser;