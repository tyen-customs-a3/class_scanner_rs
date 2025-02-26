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
            if self.check(Token::Class) {
                let class = self.parse_class()?;
                root.nested_classes.push(class);
            } else {
                self.advance(); // Skip non-class tokens
            }
        }
        
        Ok(root)
    }

    #[instrument(skip(self))]
    fn parse_class(&mut self) -> Result<ClassNode, Error> {
        self.expect_token(Token::Class)?;
        
        let name = match self.consume()? {
            Token::Identifier(name) => name,
            _ => return Err(Error::ParseError {
                message: "Expected class name".to_string(),
                location: SourceLocation::unknown()
            }),
        };
        debug!(class_name = %name, "Parsing class");

        let mut class = ClassNode::new(name.clone());
        class.file_path = self.file_path.clone();

        // Check for inheritance
        if self.check(Token::Colon) {
            self.advance();
            if self.check(Token::Public) {
                self.advance();
            }
            if let Token::Identifier(parent) = self.consume()? {
                debug!(class_name = %name, parent = %parent, "Class inheritance");
                class = class.with_parent(parent);
            }
        }

        // Handle empty class declarations (class Name;)
        if self.check(Token::Semicolon) {
            self.advance();
            debug!(class_name = %name, "Empty class declaration");
            return Ok(class);
        }

        self.expect_token(Token::LeftBrace)?;

        while !self.check(Token::RightBrace) && !self.is_at_end() {
            if self.check(Token::Class) {
                let nested_class = self.parse_class()?;
                debug!(class_name = %name, nested = %nested_class.name, "Adding nested class");
                class.nested_classes.push(nested_class);
            } else if self.check(Token::Semicolon) {
                // Skip stray semicolons
                debug!(class_name = %name, "Skipping stray semicolon");
                self.advance();
            } else {
                let property = self.parse_property()?;
                debug!(class_name = %name, property = %property.name, "Adding property");
                class.properties.insert(property.name.clone(), property);
            }
        }

        self.expect_token(Token::RightBrace)?;
        Ok(class)
    }

    #[instrument(skip(self))]
    fn parse_property(&mut self) -> Result<PropertyNode, Error> {
        trace!(token = ?self.peek(), "Starting property parse");
        let name = match self.consume()? {
            Token::Identifier(name) => name,
            token => {
                debug!(unexpected_token = ?token, "Expected property name");
                return Err(Error::ParseError { 
                    message: "Expected property name".to_string(),
                    location: SourceLocation::unknown()
                });
            }
        };

        let mut is_array = false;
        let mut operation = None;

        // Handle array syntax and operators
        if self.check(Token::ArrayMarker) {
            is_array = true;
            self.advance();
            
            if self.check(Token::PlusEquals) {
                self.advance();
                operation = Some(ArrayOperation::Append);
            } else if self.check(Token::MinusEquals) {
                self.advance();
                operation = Some(ArrayOperation::Remove);
            } else {
                self.expect_token(Token::Equals)?;
                operation = Some(ArrayOperation::Replace);
            }
        } else {
            self.expect_token(Token::Equals)?;
        }

        let (value_type, raw_value, array_values) = if is_array {
            trace!(property = %name, "Parsing array value");
            self.parse_array_value()?
        } else {
            trace!(property = %name, "Parsing single value");
            self.parse_single_value()?
        };
        
        self.expect_token(Token::Semicolon)?;

        Ok(PropertyNode {
            name,
            value_type,
            raw_value,
            operation,
            array_values,
        })
    }

    fn parse_single_value(&mut self) -> Result<(PropertyType, String, Vec<String>), Error> {
        match self.peek() {
            Token::StringLiteral(s) => {
                self.advance();
                let clean_str = s.trim_matches('"').to_string();
                Ok((PropertyType::String, clean_str, vec![]))
            }
            Token::NumberLiteral(n) => {
                self.advance();
                Ok((PropertyType::Number, n.to_string(), vec![]))
            }
            Token::BooleanLiteral(b) => {
                self.advance();
                Ok((PropertyType::Boolean, b.to_string(), vec![]))
            }
            Token::Identifier(s) => {
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
        match self.peek() {
            Token::LeftBrace => {
                self.advance();
                let mut values = Vec::new();
                
                while !self.check(Token::RightBrace) {
                    match self.consume()? {
                        Token::StringLiteral(s) => {
                            let clean_str = s.trim_matches('"').to_string();
                            values.push(clean_str);
                        },
                        Token::NumberLiteral(n) => values.push(n.to_string()),
                        Token::Identifier(s) => values.push(s),
                        _ => return Err(Error::ParseError {
                            message: "Invalid array element".to_string(),
                            location: SourceLocation::unknown()
                        }),
                    }

                    if !self.check(Token::RightBrace) {
                        self.expect_token(Token::Comma)?;
                    }
                }
                
                self.expect_token(Token::RightBrace)?;
                
                // Store array values without quotes
                Ok((PropertyType::Array, format!("{{{}}}", values.join(",")), values))
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

    fn expect_token(&mut self, expected: Token) -> Result<(), Error> {
        let expected_token = expected.clone();
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(Error::ParseError {
                message: format!("Expected token {:?}, found {:?}", expected_token, self.peek()),
                location: SourceLocation::unknown()
            })
        }
    }

    fn check(&self, token: Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current]) == std::mem::discriminant(&token)
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn peek(&self) -> Token {
        if self.is_at_end() {
            Token::EOL
        } else {
            self.tokens[self.current].clone()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}