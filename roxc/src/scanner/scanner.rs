use super::error::ScannerError;
use crate::token::Token;
use crate::token::TokenType::*;

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    done: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            done: false,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if self.is_at_end() {
            self.done = true;
            Some(Ok(Token {
                r#type: Eof,
                lexeme: None,
                line: self.line,
            }))
        } else {
            self.start = self.current;
            self.scan_token()
        }
    }
}

impl<'a> Scanner<'a> {
    fn scan_token(&mut self) -> Option<Result<Token, ScannerError>> {
        let c = self.advance()?;

        match c {
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            '{' => Some(LeftBrace),
            '}' => Some(RightBrace),
            ',' => Some(Comma),
            '.' => Some(Dot),
            '-' => Some(Minus),
            '+' => Some(Plus),
            ';' => Some(Semicolon),
            '*' => Some(Star),
            '!' => Some(if self.match_lexeme('=') {
                BangEqual
            } else {
                Bang
            }),
            '=' => Some(if self.match_lexeme('=') {
                EqualEqual
            } else {
                Equal
            }),
            '<' => Some(if self.match_lexeme('=') {
                LessEqual
            } else {
                Less
            }),
            '>' => Some(if self.match_lexeme('=') {
                GreaterEqual
            } else {
                Greater
            }),
            '/' => {
                if self.match_lexeme('/') {
                    while self.peek().iter().filter(|c| **c != '\n').next().is_some() {
                        self.advance();
                    }
                    return self.next();
                } else {
                    Some(Slash)
                }
            }
            '\n' => {
                self.line += 1;
                return self.next();
            }
            c if c.is_whitespace() => return self.next(),
            _ => None,
        }
        .map(|t| {
            Ok(Token {
                r#type: t,
                lexeme: Some(self.source[self.start..self.current].into()),
                line: self.line,
            })
        })
        .or_else(|| {
            Some(Err(ScannerError {
                lexeme: c,
                line: self.line,
                position: self.current,
                source: self.source.into(),
                message: format!("Unrecognized token"),
            }))
        })
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn match_lexeme(&mut self, expected: char) -> bool {
        if self.source.chars().nth(self.current) != Some(expected) {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next_c = self.source.chars().nth(self.current);
        self.current += 1;
        next_c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
