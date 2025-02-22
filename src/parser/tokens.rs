#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Class,
    Identifier,
    Equals,
    ArrayStart,
    ArrayEnd,
    BlockStart,
    BlockEnd,
    Semicolon,
    String,
    Number,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, position: usize) -> Self {
        Self { token_type, value, position }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyTokenType {
    Identifier,
    ArrayMarker,
    Equals,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct PropertyToken {
    pub token_type: PropertyTokenType,
    pub value: String,
    pub pos: usize,
}