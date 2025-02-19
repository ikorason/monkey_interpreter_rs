#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    // identifiers + literals
    IDENT,
    INT,

    // operators
    ASSIGN,
    PLUS,

    // delimiters
    COMMA,
    SEMICOLON,

    /// `(`
    LPAREN,
    /// `)`
    RPAREN,
    /// `{`
    LBRACE,
    /// `}`
    RBRACE,

    // keywords
    FUNCTION,
    LET,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}