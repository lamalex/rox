mod token_type;

pub use token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: Option<String>,
    pub line: usize,
}
