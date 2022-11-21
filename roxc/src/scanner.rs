use crate::{token::Token, token_type::TokenType::*, ErrorReport};

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
    type Item = Result<Token, ErrorReport>;

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
    fn scan_token(&mut self) -> Option<Result<Token, ErrorReport>> {
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
                    while self.peek().map(|c| c == '\n').is_some() {
                        self.advance();
                    }
                    return Some(self.scan_token()?);
                } else {
                    Some(Slash)
                }
            }
            ' ' | '\t' | '\r' => return self.scan_token(),
            '\n' => {
                self.line += 1;
                return self.scan_token();
            }
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
            Some(Err(ErrorReport {
                line: self.line,
                source: self.source.into(),
                message: "Unrecognized token".into(),
            }))
        })
    }

    fn peek(&self) -> Option<char> {
        return self.source.chars().nth(self.current);
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
