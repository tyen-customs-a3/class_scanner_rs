use std::str::Chars;
use log::{debug, trace};
use crate::error::Result;
use super::tokens::{Token, TokenType};

pub struct Tokenizer<'a> {
    input: Chars<'a>,
    position: usize,
    peek_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let peek_char = chars.next();
        debug!("Created new tokenizer with {} characters", input.len());
        Self {
            input: chars,
            position: 0,
            peek_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.peek_char = self.input.next();
        trace!("Advanced to position {} with peek char {:?}", self.position, self.peek_char);
    }

    fn skip_whitespace(&mut self) {
        let start_pos = self.position;
        while let Some(ch) = self.peek_char {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
        if self.position > start_pos {
            trace!("Skipped whitespace from position {} to {}", start_pos, self.position);
        }
    }

    fn skip_comments(&mut self) {
        while let Some(ch) = self.peek_char {
            if ch == '/' {
                let mut input_clone = self.input.clone();
                match input_clone.next() {
                    Some('/') => {
                        debug!("Found line comment at position {}", self.position);
                        self.advance(); // Skip first '/'
                        self.advance(); // Skip second '/'
                        while let Some(ch) = self.peek_char {
                            if ch == '\n' {
                                self.advance();
                                break;
                            }
                            self.advance();
                        }
                    }
                    Some('*') => {
                        debug!("Found block comment at position {}", self.position);
                        self.advance(); // Skip '/'
                        self.advance(); // Skip '*'
                        let mut last_char = None;
                        while let Some(ch) = self.peek_char {
                            if last_char == Some('*') && ch == '/' {
                                self.advance();
                                debug!("End of block comment at position {}", self.position);
                                break;
                            }
                            last_char = Some(ch);
                            self.advance();
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self, quote: char) -> String {
        let start_pos = self.position;
        let mut value = String::new();
        let mut is_escaped = false;

        while let Some(ch) = self.peek_char {
            if !is_escaped && ch == quote {
                self.advance();
                debug!("Read string from position {} to {}: {:?}", start_pos, self.position, value);
                break;
            }
            
            if ch == '\\' && !is_escaped {
                is_escaped = true;
            } else {
                is_escaped = false;
            }
            
            value.push(ch);
            self.advance();
        }

        value
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        let mut value = String::new();
        
        // Handle negative numbers
        if self.peek_char == Some('-') {
            value.push('-');
            self.advance();
        }

        while let Some(ch) = self.peek_char {
            if !ch.is_ascii_digit() && ch != '.' {
                break;
            }
            value.push(ch);
            self.advance();
        }
        
        debug!("Read number from position {} to {}: {}", start_pos, self.position, value);
        value
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        let mut value = String::new();
        
        while let Some(ch) = self.peek_char {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            value.push(ch);
            self.advance();
        }
        
        debug!("Read identifier from position {} to {}: {}", start_pos, self.position, value);
        value
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.skip_comments();

        let current_char = self.peek_char?;
        let current_pos = self.position;

        let token = match current_char {
            '{' => {
                self.advance();
                debug!("Found BlockStart token at position {}", current_pos);
                Token::new(TokenType::BlockStart, "{".to_string(), current_pos)
            }
            '}' => {
                self.advance();
                Token::new(TokenType::BlockEnd, "}".to_string(), current_pos)
            }
            '[' => {
                self.advance();
                Token::new(TokenType::ArrayStart, "[".to_string(), current_pos)
            }
            ']' => {
                self.advance();
                Token::new(TokenType::ArrayEnd, "]".to_string(), current_pos)
            }
            '=' => {
                self.advance();
                Token::new(TokenType::Equals, "=".to_string(), current_pos)
            }
            ';' => {
                self.advance();
                Token::new(TokenType::Semicolon, ";".to_string(), current_pos)
            }
            '"' | '\'' => {
                let quote = current_char;
                self.advance();
                Token::new(TokenType::String, self.read_string(quote), current_pos)
            }
            '0'..='9' | '-' => {
                Token::new(TokenType::Number, self.read_number(), current_pos)
            }
            _ if current_char.is_alphabetic() || current_char == '_' => {
                let value = self.read_identifier();
                let token_type = if value == "class" {
                    debug!("Found Class keyword at position {}", current_pos);
                    TokenType::Class
                } else {
                    debug!("Found Identifier token at position {}: {}", current_pos, value);
                    TokenType::Identifier
                };
                Token::new(token_type, value, current_pos)
            }
            _ => {
                self.advance();
                debug!("Skipping unknown character '{}' at position {}", current_char, current_pos);
                return self.next();
            }
        };

        Some(Ok(token))
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    debug!("Starting tokenization of input with length {}", input.len());
    let tokens = Tokenizer::new(input).collect();
    debug!("Finished tokenization");
    tokens
}
