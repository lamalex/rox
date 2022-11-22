use self::error::ScannerErrorMeta;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;

pub use error::ScannerError;

mod error;

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

        let token_type = match c {
            '(' => Ok(LeftParen),
            ')' => Ok(RightParen),
            '{' => Ok(LeftBrace),
            '}' => Ok(RightBrace),
            ',' => Ok(Comma),
            '.' => Ok(Dot),
            '-' => Ok(Minus),
            '+' => Ok(Plus),
            ';' => Ok(Semicolon),
            '*' => Ok(Star),
            '!' => Ok(if self.match_lexeme('=') {
                BangEqual
            } else {
                Bang
            }),
            '=' => Ok(if self.match_lexeme('=') {
                EqualEqual
            } else {
                Equal
            }),
            '<' => Ok(if self.match_lexeme('=') {
                LessEqual
            } else {
                Less
            }),
            '>' => Ok(if self.match_lexeme('=') {
                GreaterEqual
            } else {
                Greater
            }),
            '/' => {
                if self.match_lexeme('*') {
                    while self.peek_2().iter().any(|c| c.as_str() != "*/") {
                        if self.peek() == Some('\n') {
                            self.line += 1;
                        }
                        self.advance();
                    }
                    self.advance();
                    self.advance();
                    return self.next();
                } else if self.match_lexeme('/') {
                    while self.peek().iter().any(|c| *c != '\n') {
                        self.advance();
                    }
                    return self.next();
                } else {
                    Ok(Slash)
                }
            }
            '\n' => {
                self.line += 1;
                return self.next();
            }
            c if c.is_whitespace() => return self.next(),
            '"' => {
                let string_line_start = self.line;
                let string_pos_start = self.current;
                while self.peek().iter().any(|c| *c != '"') {
                    if self.peek().iter().any(|c| *c == '\n') {
                        self.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    Err(ScannerError::UnterminatedString(ScannerErrorMeta {
                        lexeme: None,
                        line: string_line_start,
                        position: string_pos_start,
                        source: self.source.into(),
                    }))
                } else {
                    self.advance();
                    Ok(String(self.source[self.start + 1..self.current - 1].into()))
                }
            }
            c if c.is_ascii_digit() => {
                while self.peek().iter().all(|c| c.is_ascii_digit()) {
                    self.advance();
                }

                if self.peek().iter().all(|c| *c == '.') {
                    self.advance();

                    while self.peek().iter().all(|c| c.is_ascii_digit()) {
                        self.advance();
                    }
                }

                let lexeme = &self.source[self.start..self.current];
                // We're like 99% sure at this point that we have a valid f64. Over(under)flow
                // is the only risk. If we have over or underflow I guess we'll just crash.
                match lexeme.parse() {
                    Ok(n) => Ok(Number(n)),
                    Err(_) => Err(ScannerError::Deserialization(ScannerErrorMeta {
                        lexeme: None,
                        line: self.line,
                        position: self.current,
                        source: self.source.into(),
                    })),
                }
            }
            c if c.is_alphabetic() => {
                while self.peek().iter().all(|c| c.is_alphanumeric()) {
                    self.advance();
                }

                let lexeme = &self.source[self.start..self.current];

                match TokenType::parse_keyword(lexeme) {
                    Some(t) => Ok(t),
                    None => Ok(Identifier(lexeme.into())),
                }
            }
            _ => Err(ScannerError::UnrecognizedToken(ScannerErrorMeta {
                lexeme: Some(c),
                line: self.line,
                position: self.current,
                source: self.source.into(),
            })),
        };

        match token_type {
            Ok(t) => {
                let range = match t {
                    String(_) => self.start + 1..self.current - 1, // trim surrounding quotes for strings
                    _ => self.start..self.current,
                };

                Some(Ok(Token {
                    r#type: t,
                    lexeme: Some(self.source[range].into()),
                    line: self.line,
                }))
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_2(&self) -> Option<std::string::String> {
        let next_2: std::string::String = self.source.chars().skip(self.current).take(2).collect();

        if next_2.is_empty() {
            None
        } else {
            Some(next_2)
        }
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
