#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Class,
    Enum,  
    Public,
    Private,
    Include,
    Define,
    
    // Identifiers and literals
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    ARGBColor(u8, u8, u8, f64, f64, f64, f64), // size_x, size_y, channels, r, g, b, a
    
    // Symbols
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Equals,
    PlusEquals,
    MinusEquals,
    ArrayMarker,
    
    // Special
    EOL,
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.token_type, 
            TokenType::Equals | 
            TokenType::PlusEquals | 
            TokenType::MinusEquals
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(self.token_type,
            TokenType::StringLiteral(_) |
            TokenType::NumberLiteral(_) |
            TokenType::BooleanLiteral(_) |
            TokenType::ARGBColor(_, _, _, _, _, _, _)
        )
    }

    pub fn as_string(&self) -> Option<&str> {
        match &self.token_type {
            TokenType::StringLiteral(s) | TokenType::Identifier(s) | TokenType::Comment(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_operator() {
        let token = Token::new(TokenType::Equals, 1, 1);
        assert!(token.is_operator());
        let token = Token::new(TokenType::PlusEquals, 1, 1);
        assert!(token.is_operator());
        let token = Token::new(TokenType::MinusEquals, 1, 1);
        assert!(token.is_operator());
        let token = Token::new(TokenType::Class, 1, 1);
        assert!(!token.is_operator());
        let token = Token::new(TokenType::StringLiteral("test".to_string()), 1, 1);
        assert!(!token.is_operator());
    }

    #[test]
    fn test_is_literal() {
        let token = Token::new(TokenType::StringLiteral("test".to_string()), 1, 1);
        assert!(token.is_literal());
        let token = Token::new(TokenType::NumberLiteral(42.0), 1, 1);
        assert!(token.is_literal());
        let token = Token::new(TokenType::BooleanLiteral(true), 1, 1);
        assert!(token.is_literal());
        let token = Token::new(TokenType::ARGBColor(1, 1, 3, 0.0, 0.0, 0.0, 0.0), 1, 1);
        assert!(token.is_literal());
        let token = Token::new(TokenType::Class, 1, 1);
        assert!(!token.is_literal());
        let token = Token::new(TokenType::Equals, 1, 1);
        assert!(!token.is_literal());
    }

    #[test]
    fn test_as_string() {
        let token = Token::new(TokenType::StringLiteral("test".to_string()), 1, 1);
        assert_eq!(token.as_string(), Some("test"));
        let token = Token::new(TokenType::Identifier("name".to_string()), 1, 1);
        assert_eq!(token.as_string(), Some("name"));
        let token = Token::new(TokenType::Comment("comment".to_string()), 1, 1);
        assert_eq!(token.as_string(), Some("comment"));
        let token = Token::new(TokenType::Class, 1, 1);
        assert_eq!(token.as_string(), None);
        let token = Token::new(TokenType::NumberLiteral(42.0), 1, 1);
        assert_eq!(token.as_string(), None);
    }
}