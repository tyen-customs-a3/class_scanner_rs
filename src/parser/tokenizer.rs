use std::str::Chars;
use std::marker::PhantomData;
use log::{debug, trace};
use crate::models::{Error, Result};
use super::tokens::{Token, TokenType};

// Token stream states
pub struct Unparsed;
pub struct Parsed;
pub struct Validated;

pub struct TokenStream<S = Unparsed> {
    tokens: Vec<Token>,
    position: usize,
    _state: PhantomData<S>,
}

impl TokenStream<Unparsed> {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            position: 0,
            _state: PhantomData,
        }
    }

    pub fn parse(mut self, input: &str) -> Result<TokenStream<Parsed>> {
        let tokenizer = Tokenizer::new(input);
        self.tokens = tokenizer.collect::<Result<Vec<_>>>()?;
        Ok(TokenStream {
            tokens: self.tokens,
            position: 0,
            _state: PhantomData,
        })
    }
}

impl TokenStream<Parsed> {
    pub fn validate(self) -> Result<TokenStream<Validated>> {
        // Validate token sequence
        let mut depth = 0;
        let mut in_array = false;

        for token in &self.tokens {
            match token.token_type {
                TokenType::BlockStart => depth += 1,
                TokenType::BlockEnd => {
                    depth -= 1;
                    if depth < 0 {
                        return Err(Error::Parse {
                            message: format!("Unmatched block at position {}", token.position),
                            line: None,
                            column: None
                        });
                    }
                }
                TokenType::ArrayStart => {
                    if in_array {
                        return Err(Error::Parse {
                            message: "Nested arrays not supported".to_string(),
                            line: None,
                            column: None
                        });
                    }
                    in_array = true;
                }
                TokenType::ArrayEnd => {
                    if !in_array {
                        return Err(Error::Parse {
                            message: format!("Unmatched array end at position {}", token.position),
                            line: None,
                            column: None
                        });
                    }
                    in_array = false;
                }
                _ => {}
            }
        }

        if depth != 0 {
            return Err(Error::Parse {
                message: format!("Unclosed block at position {}", self.tokens.last().map_or(0, |t| t.position)),
                line: None,
                column: None
            });
        }

        if in_array {
            return Err(Error::Parse {
                message: format!("Unclosed array at position {}", self.tokens.last().map_or(0, |t| t.position)),
                line: None,
                column: None
            });
        }

        Ok(TokenStream {
            tokens: self.tokens,
            position: 0,
            _state: PhantomData,
        })
    }
}

impl<S> TokenStream<S> {
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn expect_next(&mut self, expected: TokenType) -> Result<&Token> {
        match self.next() {
            Some(token) if token.token_type == expected => Ok(token),
            Some(token) => Err(Error::Parse {
                message: format!("Expected {:?} but found {:?} at position {}", 
                    expected, token.token_type, token.position),
                line: None,
                column: None
            }),
            None => Err(Error::Parse {
                message: "Unexpected end of tokens".to_string(),
                line: None,
                column: None
            }),
        }
    }

    pub fn consume_until(&mut self, token_type: TokenType) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.position < self.tokens.len() {
            if self.tokens[self.position].token_type == token_type {
                break;
            }
            tokens.push(self.tokens[self.position].clone());
            self.position += 1;
        }
        tokens
    }
}

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
        self.advance(); // Skip opening quote

        while let Some(ch) = self.peek_char {
            if !is_escaped {
                if ch == quote {
                    self.advance(); // Skip closing quote
                    debug!("Read string from position {} to {}: {:?}", start_pos, self.position, value);
                    break;
                }
                if ch == '\\' {
                    is_escaped = true;
                    self.advance();
                    continue;
                }
            } else {
                match ch {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    _ => value.push(ch),
                }
                is_escaped = false;
                self.advance();
                continue;
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
            if !ch.is_alphanumeric() && ch != '_' && ch != '\\' && ch != '/' && ch != '.' {
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
                Token::new(TokenType::String, self.read_string(quote), current_pos)
            }
            '0'..='9' | '-' => {
                Token::new(TokenType::Number, self.read_number(), current_pos)
            }
            _ if current_char.is_alphabetic() || current_char == '_' => {
                let value = self.read_identifier();
                let token_type = match value.to_lowercase().as_str() {
                    "class" => TokenType::Class,
                    "true" | "false" => TokenType::Boolean,
                    _ => TokenType::Identifier,
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
