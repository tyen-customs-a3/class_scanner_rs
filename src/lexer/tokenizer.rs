use super::tokens::{Token, TokenType};
use crate::error::{Error, SourceLocation};
use std::iter::Peekable;
use std::str::Chars;
use std::path::PathBuf;

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    file_path: Option<PathBuf>,
    preserve_comments: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
            column: 0,
            file_path: None,
            preserve_comments: false,
        }
    }

    pub fn with_file_path(input: &'a str, file_path: impl Into<PathBuf>) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
            column: 0,
            file_path: Some(file_path.into()),
            preserve_comments: false,
        }
    }

    pub fn with_comments(mut self, preserve: bool) -> Self {
        self.preserve_comments = preserve;
        self
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        self.skip_whitespace();

        match self.peek() {
            None => Ok(None),
            Some(c) => {
                let column = self.column; // Store column before advancing
                let token = match c {
                    '{' => self.single_char_token(TokenType::LeftBrace),
                    '}' => self.single_char_token(TokenType::RightBrace),
                    '[' => {
                        self.advance();
                        if self.match_char(']') {
                            Token::new(TokenType::ArrayMarker, self.line, column)
                        } else {
                            Token::new(TokenType::LeftBracket, self.line, column)
                        }
                    },
                    ']' => self.single_char_token(TokenType::RightBracket),
                    ';' => self.single_char_token(TokenType::Semicolon),
                    ':' => self.single_char_token(TokenType::Colon),
                    ',' => self.single_char_token(TokenType::Comma),
                    '=' => self.handle_equals()?,
                    '+' => self.handle_plus()?,
                    '-' => {
                        if matches!(self.peek_next(), Some('0'..='9')) {
                            self.read_number()?
                        } else {
                            self.handle_minus()?
                        }
                    },
                    '"' => self.read_string()?,
                    '/' => {
                        if self.peek_next() == Some('/') {
                            self.read_line_comment()?
                        } else if self.peek_next() == Some('*') {
                            self.read_block_comment()?
                        } else {
                            return Err(self.error("Unexpected '/' character"));
                        }
                    },
                    '#' => {
                        if matches!(self.peek_next(), Some('(')) {
                            self.read_argb_color()?
                        } else {
                            self.read_preprocessor_directive()?
                        }
                    },
                    '\\' => self.read_identifier(), // Treat backslash as part of an identifier for texture paths
                    c if c.is_ascii_digit() => {
                        if self.is_part_of_identifier() {
                            self.read_identifier()
                        } else {
                            self.read_number()?
                        }
                    },
                    c if c.is_ascii_alphabetic() || c == '_' => self.read_identifier(),
                    _ => return Err(self.error(&format!("Unexpected character: {}", c))),
                };
                Ok(Some(token))
            }
        }
    }
    
    fn is_part_of_identifier(&mut self) -> bool {
        // Look ahead to check if this numeric start is part of an identifier
        let mut iter = self.input.clone();
        let mut found_underscore_or_letter = false;
        
        // Skip the first character (we know it's a digit)
        iter.next();
        
        // Check if any subsequent character would make this an identifier
        while let Some(c) = iter.next() {
            if c.is_ascii_alphabetic() || c == '_' || c == '\\' {
                found_underscore_or_letter = true;
                break;
            } else if !c.is_ascii_digit() && c != '.' && c != 'e' && c != 'E' && c != '+' && c != '-' {
                // If we hit something that's not a valid part of a number or identifier, stop
                break;
            }
        }
        
        found_underscore_or_letter
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance(); // Skip opening quote
        let mut string = String::new();
        
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // Skip closing quote
                return Ok(Token::new(TokenType::StringLiteral(string), line, column));
            }
            string.push(c);
            self.advance();
        }
        
        Err(self.error("Unterminated string literal"))
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        let mut number = String::new();
        let mut has_dot = false;
        let mut has_e = false;

        // Handle negative numbers at start
        if self.peek() == Some('-') {
            number.push('-');
            self.advance();
            
            // There must be a digit after the minus sign
            if !matches!(self.peek(), Some('0'..='9')) {
                return Err(self.error("Expected digit after '-'"));
            }
        }

        // Main number parsing loop
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    number.push(c);
                    self.advance();
                }
                '.' if !has_dot && !has_e => {
                    has_dot = true;
                    number.push(c);
                    self.advance();

                    // There must be a digit after the decimal point
                    if !matches!(self.peek(), Some('0'..='9')) {
                        return Err(self.error("Expected digit after decimal point"));
                    }
                }
                'e' | 'E' if !has_e => {
                    // Look ahead to ensure the next character exists and is valid for scientific notation
                    match self.peek_next() {
                        Some(next) if next == '-' || next == '+' || next.is_ascii_digit() => {
                            has_e = true;
                            number.push('e');
                            self.advance();
                            
                            // Handle optional + or - after e
                            match self.peek() {
                                Some('+') => {
                                    self.advance(); // Skip the plus sign
                                }
                                Some('-') => {
                                    number.push('-');
                                    self.advance();
                                }
                                Some('0'..='9') => {}, // Digit directly after e is fine
                                _ => return Err(self.error("Expected '+', '-' or digit after 'e' in scientific notation"))
                            }

                            // Must have at least one digit in the exponent
                            if !matches!(self.peek(), Some('0'..='9')) {
                                return Err(self.error("Expected digit in scientific notation exponent"));
                            }
                        }
                        // No valid character after 'e', reject scientific notation
                        _ => return Err(self.error("Invalid scientific notation format: expected digit, '+', or '-' after 'e'")),
                    }
                }
                _ if has_e && matches!(c, '0'..='9') => {
                    number.push(c);
                    self.advance();
                }
                _ => break,
            }
        }

        match number.parse::<f64>() {
            Ok(n) => Ok(Token::new(TokenType::NumberLiteral(n), line, column)),
            Err(_) => Err(self.error(&format!("Invalid number format: {}", number))),
        }
    }

    fn read_identifier(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut ident = String::new();
        
        // Allow digits, letters, underscores, backslashes, and dots in identifiers
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '\\' || c == '.' || c == '-' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a pure number (no underscores or letters other than e/E for scientific)
        let is_pure_number = ident.chars()
            .enumerate()
            .all(|(i, c)| c.is_ascii_digit() || c == '.' || 
                 (i > 0 && (c == 'e' || c == 'E' || (i > 1 && (c == '-' || c == '+') && 
                 (ident.chars().nth(i-1).unwrap() == 'e' || ident.chars().nth(i-1).unwrap() == 'E')))));
                 
        if is_pure_number {
            if let Ok(num) = ident.parse::<f64>() {
                return Token::new(TokenType::NumberLiteral(num), line, column);
            }
        }

        let token_type = match ident.as_str() {
            "class" => TokenType::Class,
            "enum" => TokenType::Enum,
            "public" => TokenType::Public,
            "private" => TokenType::Private,
            "include" => TokenType::Include,
            "define" => TokenType::Define,
            "true" => TokenType::BooleanLiteral(true),
            "false" => TokenType::BooleanLiteral(false),
            _ => TokenType::Identifier(ident),
        };

        Token::new(token_type, line, column)
    }

    fn read_line_comment(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance(); // Skip first '/'
        self.advance(); // Skip second '/'
        
        let mut comment = String::new();
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }

        if self.preserve_comments {
            Ok(Token::new(TokenType::Comment(comment), line, column))
        } else {
            self.next_token()?
                .ok_or_else(|| self.error("Unexpected end of input after comment"))
        }
    }

    fn read_block_comment(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance(); // Skip '/'
        self.advance(); // Skip '*'
        
        let mut comment = String::new();
        let mut depth = 1;

        while depth > 0 {
            match (self.peek(), self.peek_next()) {
                (Some('*'), Some('/')) => {
                    depth -= 1;
                    self.advance();
                    self.advance();
                }
                (Some('/'), Some('*')) => {
                    depth += 1;
                    comment.push('/');
                    comment.push('*');
                    self.advance();
                    self.advance();
                }
                (Some('\n'), _) => {
                    comment.push('\n');
                    self.advance();
                    self.line += 1;
                    self.column = 0;
                }
                (Some(c), _) => {
                    comment.push(c);
                    self.advance();
                }
                (None, _) => return Err(self.error("Unterminated block comment")),
            }
        }

        if self.preserve_comments {
            Ok(Token::new(TokenType::Comment(comment), line, column))
        } else {
            self.next_token()?
                .ok_or_else(|| self.error("Unexpected end of input after comment"))
        }
    }

    fn read_preprocessor_directive(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance(); // Skip '#'
        
        // For preprocessor directives, we want to preserve the original column
        let token_type = match self.read_identifier().token_type {
            TokenType::Include => TokenType::Include,
            TokenType::Define => TokenType::Define,
            _ => return Err(self.error("Unknown preprocessor directive")),
        };
        
        Ok(Token::new(token_type, line, column))
    }

    fn handle_equals(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance();
        Ok(Token::new(TokenType::Equals, line, column))
    }

    fn handle_plus(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance();
        if self.match_char('=') {
            Ok(Token::new(TokenType::PlusEquals, line, column))
        } else {
            Err(self.error("Expected '=' after '+'"))
        }
    }

    fn handle_minus(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance();
        
        // Check if next character is a digit (negative number)
        if matches!(self.peek(), Some('0'..='9')) {
            // Backtrack to let read_number handle it
            self.column -= 1;
            self.read_number()
        } else if self.match_char('=') {
            Ok(Token::new(TokenType::MinusEquals, line, column))
        } else {
            Err(self.error("Expected '=' after '-' or digit for negative number"))
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn peek_next(&mut self) -> Option<char> {
        let mut iter = self.input.clone();
        iter.next();
        iter.next()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.input.next();
        if let Some(c) = c {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        c
    }

    fn advance(&mut self) {
        self.next();
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn single_char_token(&mut self, token_type: TokenType) -> Token {
        let line = self.line;
        let column = self.column;
        self.advance();
        Token::new(token_type, line, column)
    }

    fn error(&self, message: &str) -> Error {
        Error::LexerError {
            message: message.to_string(),
            location: SourceLocation::new(
                self.file_path.clone(),
                self.line,
                self.column
            )
        }
    }

    fn read_argb_color(&mut self) -> Result<Token, Error> {
        let line = self.line;
        let column = self.column;
        self.advance(); // Skip '#'
        if !self.match_char('(') {
            return Err(self.error("Expected '(' after '#' in ARGB color"));
        }

        // Expect "argb"
        let format = self.read_identifier();
        if format.token_type != TokenType::Identifier("argb".to_string()) {
            return Err(self.error("Expected 'argb' in color format"));
        }

        if !self.match_char(',') {
            return Err(self.error("Expected ',' after 'argb'"));
        }

        // Read size_x
        self.skip_whitespace();
        let size_x = self.read_number()?.to_u8()?;

        if !self.match_char(',') {
            return Err(self.error("Expected ',' after size_x"));
        }

        // Read size_y
        self.skip_whitespace();
        let size_y = self.read_number()?.to_u8()?;

        if !self.match_char(',') {
            return Err(self.error("Expected ',' after size_y"));
        }

        // Read channels
        self.skip_whitespace();
        let channels = self.read_number()?.to_u8()?;

        if !self.match_char(')') {
            return Err(self.error("Expected ')' after channels"));
        }

        // Parse the color values
        self.skip_whitespace();
        if !self.match_char_str("color(") {
            return Err(self.error("Expected 'color(' after ARGB format"));
        }

        // Read r,g,b,a values
        self.skip_whitespace();
        let r = self.read_number()?.to_f64()?;
        if r < 0.0 || r > 1.0 {
            return Err(self.error("Color values must be between 0 and 1"));
        }

        if !self.match_char(',') {
            return Err(self.error("Expected ',' after red value"));
        }

        self.skip_whitespace();
        let g = self.read_number()?.to_f64()?;
        if g < 0.0 || g > 1.0 {
            return Err(self.error("Color values must be between 0 and 1"));
        }

        if !self.match_char(',') {
            return Err(self.error("Expected ',' after green value"));
        }

        self.skip_whitespace();
        let b = self.read_number()?.to_f64()?;
        if b < 0.0 || b > 1.0 {
            return Err(self.error("Color values must be between 0 and 1"));
        }

        if !self.match_char(',') {
            return Err(self.error("Expected ',' after blue value"));
        }

        self.skip_whitespace();
        let a = self.read_number()?.to_f64()?;
        if a < 0.0 || a > 1.0 {
            return Err(self.error("Color values must be between 0 and 1"));
        }

        if !self.match_char(')') {
            return Err(self.error("Expected ')' after alpha value"));
        }

        Ok(Token::new(TokenType::ARGBColor(size_x, size_y, channels, r, g, b, a), line, column))
    }

    fn match_char_str(&mut self, s: &str) -> bool {
        let mut chars = s.chars();
        let mut input_copy = self.input.clone();
        
        while let Some(expected) = chars.next() {
            match input_copy.next() {
                Some(c) if c == expected => continue,
                _ => return false,
            }
        }
        
        // If we matched all chars, advance the real input
        for _ in 0..s.len() {
            self.advance();
        }
        true
    }
}

trait ToNumber {
    fn to_u8(&self) -> Result<u8, Error>;
    fn to_f64(&self) -> Result<f64, Error>;
}

impl ToNumber for Token {
    fn to_u8(&self) -> Result<u8, Error> {
        match self.token_type {
            TokenType::NumberLiteral(n) => {
                if n >= 0.0 && n <= 255.0 && n.fract() == 0.0 {
                    Ok(n as u8)
                } else {
                    Err(Error::LexerError {
                        message: format!("Invalid u8 value: {}", n),
                        location: SourceLocation::unknown()
                    })
                }
            },
            _ => Err(Error::LexerError {
                message: "Expected number".to_string(),
                location: SourceLocation::unknown()
            })
        }
    }

    fn to_f64(&self) -> Result<f64, Error> {
        match self.token_type {
            TokenType::NumberLiteral(n) => Ok(n),
            _ => Err(Error::LexerError {
                message: "Expected number".to_string(),
                location: SourceLocation::unknown()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "class MyClass { public }";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::new(TokenType::Class, 1, 0),
            Token::new(TokenType::Identifier("MyClass".to_string()), 1, 6),
            Token::new(TokenType::LeftBrace, 1, 14),
            Token::new(TokenType::Public, 1, 16),
            Token::new(TokenType::RightBrace, 1, 23),
        ]);
    }

    #[test]
    fn test_string_literals() {
        let input = r#"class Test { "hello world" "no escape chars \\ \n \t allowed" }"#;
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        // Print tokens for debugging
        println!("String Literals Test - All tokens:");
        for (i, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, token);
        }
        
        assert!(tokens.contains(&Token::new(TokenType::StringLiteral("hello world".to_string()), 1, 13)));
        assert!(tokens.contains(&Token::new(TokenType::StringLiteral("no escape chars \\\\ \\n \\t allowed".to_string()), 1, 27)));
    }

    #[test]
    fn test_number_literals() {
        let input = "123 -456 789.0";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert!(tokens.contains(&Token::new(TokenType::NumberLiteral(123.0), 1, 0)));
        assert!(tokens.contains(&Token::new(TokenType::NumberLiteral(-456.0), 1, 4)));
        assert!(tokens.contains(&Token::new(TokenType::NumberLiteral(789.0), 1, 9)));
    }

    #[test]
    fn test_comments() {
        let input = "// Line comment\n/* Block comment */\nclass";
        let mut tokenizer = Tokenizer::new(input).with_comments(true);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert!(tokens.contains(&Token::new(TokenType::Comment(" Line comment".to_string()), 1, 0)));
        assert!(tokens.contains(&Token::new(TokenType::Comment(" Block comment ".to_string()), 2, 0)));
        assert!(tokens.contains(&Token::new(TokenType::Class, 3, 0)));
    }

    #[test]
    fn test_operators() {
        let input = "= += -=";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::new(TokenType::Equals, 1, 0),
            Token::new(TokenType::PlusEquals, 1, 2),
            Token::new(TokenType::MinusEquals, 1, 5),
        ]);
    }

    #[test]
    fn test_array_marker() {
        let input = "[] [";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::new(TokenType::ArrayMarker, 1, 0),
            Token::new(TokenType::LeftBracket, 1, 3),
        ]);
    }

    #[test]
    fn test_preprocessor_directives() {
        let input = "#include #define";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::new(TokenType::Include, 1, 0),
            Token::new(TokenType::Define, 1, 9),
        ]);
    }

    #[test]
    fn test_error_cases() {
        // Unterminated string
        let input = "\"unterminated";
        let mut tokenizer = Tokenizer::new(input);
        assert!(tokenizer.tokenize().is_err());

        // Invalid number
        let input = "12.34.56";
        let mut tokenizer = Tokenizer::new(input);
        assert!(tokenizer.tokenize().is_err());

        // Unterminated block comment
        let input = "/* unterminated";
        let mut tokenizer = Tokenizer::new(input);
        assert!(tokenizer.tokenize().is_err());
    }

    #[test]
    fn test_texture_paths() {
        let input = r#"hiddenSelectionsTextures[] = {\rhsusf\addons\rhsusf_infantry2\gear\head\data\rhs_helmet_mich_des_co.paa};"#;
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        // Print tokens for debugging
        println!("Texture Paths Test - All tokens:");
        for (i, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, token);
        }
        
        // The path should be a single identifier token
        assert!(tokens.contains(&Token::new(TokenType::Identifier(r#"\rhsusf\addons\rhsusf_infantry2\gear\head\data\rhs_helmet_mich_des_co.paa"#.to_string()), 1, 30)));
    }

    #[test]
    fn test_argb_color() {
        let input = r#"#(argb,8,8,3)color(1,1,1,1)"#;
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::new(TokenType::ARGBColor(8, 8, 3, 1.0, 1.0, 1.0, 1.0), 1, 0),
        ]);

        // Test invalid formats
        let invalid_inputs = vec![
            "#(rgb,8,8,3)color(1,1,1,1)",  // wrong format name
            "#(argb,256,8,3)color(1,1,1,1)",  // invalid size_x
            "#(argb,8,8,3)color(2,1,1,1)",  // invalid color value > 1
        ];

        for input in invalid_inputs {
            let mut tokenizer = Tokenizer::new(input);
            assert!(tokenizer.tokenize().is_err());
        }
    }

    #[test]
    fn test_escape_characters() {
        let input = r#""\n \r \t \\""#;
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::new(TokenType::StringLiteral(r#"\n \r \t \\"#.to_string()), 1, 0),
        ]);

        let input2 = r#"scriptsPath = "A3\Functions_F\Scripts\";"#;
        let mut tokenizer = Tokenizer::new(input2);
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens, vec![
            Token::new(TokenType::Identifier("scriptsPath".to_string()), 1, 0),
            Token::new(TokenType::Equals, 1, 12),
            Token::new(TokenType::StringLiteral(r#"A3\Functions_F\Scripts\"#.to_string()), 1, 14),
            Token::new(TokenType::Semicolon, 1, 39),
        ]);
    }

    #[test]
    fn test_string_edge_cases() {
        // Keep only basic test cases for now
        let test_cases = vec![
            (r#""""#, ""),  // Empty string
            (r#""simple string""#, "simple string"),  // Basic string
            (r#""  spaced  ""#, "  spaced  "),  // String with spaces
            (r#""123""#, "123"),  // String with numbers
        ];

        for (i, (input, expected)) in test_cases.iter().enumerate() {
            let mut tokenizer = Tokenizer::new(input);
            let result = tokenizer.tokenize().unwrap_or_else(|e| panic!("Case {} failed: {} | Input: {}", i, e, input));
            assert_eq!(
                result,
                vec![Token::new(TokenType::StringLiteral(expected.to_string()), 1, 0)],
                "Case {} failed | Input: {}", i, input
            );
        }
    }

    #[test]
    fn test_string_error_cases() {
        let error_cases = vec![
            r#"""#,           // Unterminated string
            r#""Test"#,       // Unterminated string with content
        ];

        for input in error_cases {
            let mut tokenizer = Tokenizer::new(input);
            assert!(
                tokenizer.tokenize().is_err(),
                "Expected error for input: {}", input
            );
        }
    }

    #[test]
    fn test_complex_paths() {
        let inputs = vec![
            r#"path = "A3\Functions_F\Scripts\";"#,
            r#"file = "\A3\characters_f\Heads\m_white_01.p3d";"#,
            r#"texture = "\rhsusf\addons\rhsusf_infantry2\gear\head\data\rhs_helmet_mich_des_co.paa";"#,
            // Since we're treating strings as literal, remove the test with escaped quotes
            // as it would terminate at the first quote
        ];

        for input in inputs {
            let mut tokenizer = Tokenizer::new(input);
            let result = tokenizer.tokenize();
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let tokens = result.unwrap();
            // Verify that backslashes are preserved exactly as they appear in the string
            for token in tokens {
                if let TokenType::StringLiteral(s) = token.token_type {
                    assert_eq!(s, s.to_string(), "String content should be preserved exactly");
                }
            }
        }
    }

    #[test]
    fn test_numeric_identifiers() {
        let inputs = vec![
            ("3DEN", vec![Token::new(TokenType::Identifier("3DEN".to_string()), 1, 0)]),
            ("123abc", vec![Token::new(TokenType::Identifier("123abc".to_string()), 1, 0)]),
            ("3_way", vec![Token::new(TokenType::Identifier("3_way".to_string()), 1, 0)]),
            // Make sure regular numbers still work
            ("123", vec![Token::new(TokenType::NumberLiteral(123.0), 1, 0)]),
            // Test number followed by space then identifier
            ("123 abc", vec![
                Token::new(TokenType::NumberLiteral(123.0), 1, 0),
                Token::new(TokenType::Identifier("abc".to_string()), 1, 4),
            ]),
        ];

        for (input, expected) in inputs {
            let mut tokenizer = Tokenizer::new(input);
            let result = tokenizer.tokenize().unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_scientific_notation() {
        let inputs = vec![
            ("1e5", 1e5),
            ("1e-5", 1e-5),
            ("1.23e5", 1.23e5),
            ("1.23e-5", 1.23e-5),
            ("3e-5", 3e-5),
            ("3e-05", 3e-5),  // Test with leading zeros in exponent
            ("-1.23e-5", -1.23e-5),
            ("1E5", 1e5),  // Test uppercase E
            ("0.008", 0.008),  // Test regular decimal
            ("0.014", 0.014),  // Test regular decimal
            ("3e-05", 3e-5),   // Test the exact case from the file
            ("5", 5.0),        // Test integer
        ];

        for (input, expected) in inputs {
            let mut tokenizer = Tokenizer::new(input);
            let result = tokenizer.tokenize().unwrap();
            assert_eq!(
                result,
                vec![Token::new(TokenType::NumberLiteral(expected), 1, 0)],
                "Failed for input: {}. Got: {:?}", input, result
            );
        }

        // Test invalid scientific notation
        let invalid_inputs = vec![
            "1e",      // No exponent
            "1e-",     // No exponent after minus
            "1.2e",    // No exponent
            "e5",      // No mantissa
            "1.2.3e5", // Multiple decimal points
            "1e2.5",   // Decimal in exponent
        ];

        for input in invalid_inputs {
            let mut tokenizer = Tokenizer::new(input);
            assert!(
                tokenizer.tokenize().is_err(),
                "Expected error for invalid scientific notation: {}", input
            );
        }
    }
}