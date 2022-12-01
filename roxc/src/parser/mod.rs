use crate::token::Token;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Expr::Binary(left, operator, right) => {
                format!(
                    "({} {} {})",
                    operator.lexeme.clone().unwrap_or_default(),
                    left,
                    right
                )
            }
            Expr::Grouping(expr) => format!("(group {})", expr),
            Expr::Literal(lit) => lit.to_string(),
            Expr::Unary(operator, right) => {
                format!("({} {}", operator.lexeme.clone().unwrap_or_default(), right)
            }
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(std::string::String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Cow<str> = match self {
            Literal::Number(n) => Cow::Owned(n.to_string()),
            Literal::String(s) => Cow::Borrowed(s),
            Literal::Boolean(b) => Cow::Owned(b.to_string()),
            Literal::Nil => Cow::Borrowed("nil"),
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenType};

    use super::{Expr, Literal};

    #[test]
    fn pretty_print_expr_tree() {
        let e = Expr::Binary(
            Expr::Unary(
                Token {
                    r#type: TokenType::Minus,
                    lexeme: Some("-".to_string()),
                    line: 1,
                },
                Expr::Literal(Literal::Number(123.).into()).into(),
            )
            .into(),
            Token {
                r#type: TokenType::Star,
                lexeme: Some("*".to_string()),
                line: 1,
            },
            Expr::Grouping(Expr::Literal(Literal::Number(45.67)).into()).into(),
        );

        println!("{e}");
    }
}
