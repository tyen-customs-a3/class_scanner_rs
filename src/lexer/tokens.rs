#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Class,
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

impl Token {
    pub fn is_operator(&self) -> bool {
        matches!(self, 
            Token::Equals | 
            Token::PlusEquals | 
            Token::MinusEquals
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(self,
            Token::StringLiteral(_) |
            Token::NumberLiteral(_) |
            Token::BooleanLiteral(_) |
            Token::ARGBColor(_, _, _, _, _, _, _)
        )
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Token::StringLiteral(s) | Token::Identifier(s) | Token::Comment(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_operator() {
        assert!(Token::Equals.is_operator());
        assert!(Token::PlusEquals.is_operator());
        assert!(Token::MinusEquals.is_operator());
        assert!(!Token::Class.is_operator());
        assert!(!Token::StringLiteral("test".to_string()).is_operator());
    }

    #[test]
    fn test_is_literal() {
        assert!(Token::StringLiteral("test".to_string()).is_literal());
        assert!(Token::NumberLiteral(42.0).is_literal());
        assert!(Token::BooleanLiteral(true).is_literal());
        assert!(Token::ARGBColor(1, 1, 3, 0.0, 0.0, 0.0, 0.0).is_literal());
        assert!(!Token::Class.is_literal());
        assert!(!Token::Equals.is_literal());
    }

    #[test]
    fn test_as_string() {
        assert_eq!(Token::StringLiteral("test".to_string()).as_string(), Some("test"));
        assert_eq!(Token::Identifier("name".to_string()).as_string(), Some("name"));
        assert_eq!(Token::Comment("comment".to_string()).as_string(), Some("comment"));
        assert_eq!(Token::Class.as_string(), None);
        assert_eq!(Token::NumberLiteral(42.0).as_string(), None);
    }
}