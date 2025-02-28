use crate::lexer::tokens::TokenType;
use crate::lexer::{Token, Tokenizer};
use crate::ast::{ClassNode, PropertyNode, PropertyType, AccessModifier};
use crate::error::{Error, SourceLocation};
use crate::operations::arrays::ArrayOperation;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{debug, trace, instrument};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    file_path: Option<PathBuf>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            file_path: None,
        }
    }

    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = fs::read_to_string(&path)?;
        let tokens = crate::lexer::Tokenizer::with_file_path(&content, path.as_ref()).tokenize()?;
        Ok(Self {
            tokens,
            current: 0,
            file_path: Some(path.as_ref().to_path_buf()),
        })
    }

    pub fn parse(&mut self) -> Result<ClassNode, Error> {
        // Create a root node to hold all top-level classes
        let mut root = ClassNode::new("".to_string());
        
        while !self.is_at_end() {
            if self.check(TokenType::Class) {
                let class = self.parse_class()?;
                root.nested_classes.push(class);
            } else if self.check(TokenType::Enum) {
                // Skip over enum blocks since we don't process them
                self.skip_enum_block()?;
            } else {
                self.advance(); // Skip non-class tokens
            }
        }
        
        if root.nested_classes.is_empty() {
            // No classes found, but that's okay if we found other valid content
            debug!("No classes found in parsed content");
        }
        
        Ok(root)
    }

    #[instrument(skip(self))]
    fn parse_class(&mut self) -> Result<ClassNode, Error> {
        self.expect_token(TokenType::Class)?;
        
        let name_token = self.consume()?;
        match &name_token.token_type {
            TokenType::Identifier(name) => {
                let mut class = ClassNode::new(name.clone());
                class.file_path = self.file_path.clone();

                // Check for inheritance
                if self.check(TokenType::Colon) {
                    self.advance();
                    if self.check(TokenType::Public) {
                        self.advance();
                    }
                    if let TokenType::Identifier(parent) = self.consume()?.token_type {
                        debug!(class_name = %name, parent = %parent, "Class inheritance");
                        class = class.with_parent(parent);
                    }
                }

                // Handle empty class declarations (class Name;)
                if self.check(TokenType::Semicolon) {
                    self.advance();
                    debug!(class_name = %name, "Empty class declaration");
                    return Ok(class);
                }

                self.expect_token(TokenType::LeftBrace)?;

                while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                    if self.check(TokenType::Class) {
                        let nested_class = self.parse_class()?;
                        debug!(class_name = %name, nested = %nested_class.name, "Adding nested class");
                        class.nested_classes.push(nested_class);
                    } else if self.check(TokenType::Semicolon) {
                        // Skip stray semicolons
                        debug!(class_name = %name, "Skipping stray semicolon");
                        self.advance();
                    } else {
                        let property = self.parse_property()?;
                        debug!(class_name = %name, property = %property.name, "Adding property");
                        class.properties.insert(property.name.clone(), property);
                    }
                }

                self.expect_token(TokenType::RightBrace)?;
                Ok(class)
            },
            _ => Err(Error::ParseError {
                message: "Expected class name".to_string(),
                location: SourceLocation::new(self.file_path.clone(), name_token.line, name_token.column)
            }),
        }
    }

    #[instrument(skip(self))]
    fn parse_property(&mut self) -> Result<PropertyNode, Error> {
        trace!(token = ?self.peek(), "Starting property parse");
        let name_token = self.consume()?;
        let name = match &name_token.token_type {
            TokenType::Identifier(name) => name.clone(),
            token => {
                debug!(unexpected_token = ?token, "Expected property name");
                return Err(Error::ParseError { 
                    message: "Expected property name".to_string(),
                    location: SourceLocation::new(self.file_path.clone(), name_token.line, name_token.column)
                });
            }
        };

        let mut is_array = false;
        let mut operation = None;

        // Handle array syntax and operators
        if self.check(TokenType::ArrayMarker) {
            is_array = true;
            self.advance();
            
            if self.check(TokenType::PlusEquals) {
                self.advance();
                operation = Some(ArrayOperation::Append);
            } else if self.check(TokenType::MinusEquals) {
                self.advance();
                operation = Some(ArrayOperation::Remove);
            } else {
                self.expect_token(TokenType::Equals)?;
                operation = Some(ArrayOperation::Replace);
            }
        } else {
            self.expect_token(TokenType::Equals)?;
        }

        let (value_type, raw_value, array_values) = if is_array {
            trace!(property = %name, "Parsing array value");
            self.parse_array_value()?
        } else {
            trace!(property = %name, "Parsing single value");
            self.parse_single_value()?
        };
        
        self.expect_token(TokenType::Semicolon)?;

        Ok(PropertyNode {
            name,
            value_type,
            raw_value,
            operation,
            array_values,
        })
    }

    fn parse_single_value(&mut self) -> Result<(PropertyType, String, Vec<String>), Error> {
        match self.peek().token_type {
            TokenType::StringLiteral(s) => {
                self.advance();
                let clean_str = s.trim_matches('"').to_string();
                Ok((PropertyType::String, clean_str, vec![]))
            }
            TokenType::NumberLiteral(n) => {
                self.advance();
                Ok((PropertyType::Number, n.to_string(), vec![]))
            }
            TokenType::BooleanLiteral(b) => {
                self.advance();
                Ok((PropertyType::Boolean, b.to_string(), vec![]))
            }
            TokenType::Identifier(s) => {
                self.advance();
                Ok((PropertyType::String, s, vec![]))
            }
            _ => Err(Error::ParseError {
                message: "Expected value".to_string(),
                location: SourceLocation::unknown()
            }),
        }
    }

    fn parse_array_value(&mut self) -> Result<(PropertyType, String, Vec<String>), Error> {
        match self.peek().token_type {
            TokenType::LeftBrace => {
                self.advance();
                let mut values = Vec::new();
                
                while !self.check(TokenType::RightBrace) {
                    let value = match self.consume()?.token_type {
                        TokenType::StringLiteral(s) => s.trim_matches('"').to_string(),
                        TokenType::NumberLiteral(n) => n.to_string(),
                        TokenType::Identifier(s) => s,
                        _ => return Err(Error::ParseError {
                            message: "Invalid array element".to_string(),
                            location: SourceLocation::unknown()
                        }),
                    };
                    values.push(value);

                    if !self.check(TokenType::RightBrace) {
                        self.expect_token(TokenType::Comma)?;
                    }
                }
                
                self.expect_token(TokenType::RightBrace)?;
                
                // Format raw value without extra quotes
                let raw_value = format!("{{{}}}", values.join(","));
                
                Ok((PropertyType::Array, raw_value, values))
            }
            _ => Err(Error::ParseError {
                message: "Expected array value".to_string(),
                location: SourceLocation::unknown()
            }),
        }
    }

    fn consume(&mut self) -> Result<Token, Error> {
        trace!(current_token = ?self.peek(), "Consuming token");
        if self.is_at_end() {
            Err(Error::ParseError {
                message: "Unexpected end of input".to_string(),
                location: SourceLocation::unknown()
            })
        } else {
            let token = self.tokens[self.current].clone();
            self.advance();
            trace!(consumed_token = ?token, "Token consumed");
            Ok(token)
        }
    }

    fn expect_token(&mut self, expected_type: TokenType) -> Result<(), Error> {
        let token = self.peek();
        if std::mem::discriminant(&token.token_type) == std::mem::discriminant(&expected_type) {
            self.advance();
            Ok(())
        } else {
            Err(Error::ParseError {
                message: format!("Expected token {:?}, found {:?}", expected_type, token.token_type),
                location: SourceLocation::new(self.file_path.clone(), token.line, token.column)
            })
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current].token_type) == std::mem::discriminant(&token_type)
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn peek(&self) -> Token {
        if self.is_at_end() {
            Token::new(TokenType::EOL, 0, 0)
        } else {
            self.tokens[self.current].clone()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn skip_enum_block(&mut self) -> Result<(), Error> {
        self.advance(); // Skip 'enum' token
        
        // Skip until we find the opening brace
        while !self.check(TokenType::LeftBrace) && !self.is_at_end() {
            self.advance();
        }
        
        if self.is_at_end() {
            return Err(Error::ParseError {
                message: "Unexpected end of file while skipping enum block".to_string(),
                location: SourceLocation::unknown()
            });
        }
        
        // Skip the opening brace
        self.advance();
        debug!("Starting to skip enum block");
        
        // Keep track of nested braces
        let mut depth = 1;
        let mut enum_values = Vec::new();
        
        while depth > 0 && !self.is_at_end() {
            match self.peek().token_type {
                TokenType::LeftBrace => {
                    depth += 1;
                    self.advance();
                }
                TokenType::RightBrace => {
                    depth -= 1;
                    self.advance();
                    if depth == 0 {
                        // After closing brace, expect semicolon
                        if self.check(TokenType::Semicolon) {
                            self.advance();
                        } else {
                            debug!("Warning: No semicolon after enum block");
                        }
                    }
                }
                TokenType::Identifier(name) if depth == 1 => {
                    enum_values.push(name.clone());
                    self.advance();
                    // Skip any value assignment (=)
                    if self.check(TokenType::Equals) {
                        self.advance();
                        if let TokenType::NumberLiteral(_) = self.peek().token_type {
                            self.advance();
                        }
                    }
                    // Skip comma
                    if self.check(TokenType::Comma) {
                        self.advance();
                    }
                }
                _ => self.advance()
            }
        }
        
        if depth > 0 {
            return Err(Error::ParseError {
                message: "Unclosed enum block".to_string(),
                location: SourceLocation::unknown()
            });
        }
        
        debug!("Skipped enum block with {} values", enum_values.len());
        if !enum_values.is_empty() {
            trace!("Enum values: {:?}", enum_values);
        }
        
        Ok(())
    }
}