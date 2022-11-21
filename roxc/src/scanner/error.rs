use std::{error, fmt};

#[derive(Debug)]
pub struct ScannerError {
    pub lexeme: char,
    pub line: usize,
    pub position: usize,
    pub source: String,
    pub message: String,
}

impl error::Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = self.source.split('\n');
        let offset = lines.clone().take(self.line - 1).collect::<String>().len();
        let col = self.position - offset - self.line;

        let offending_line: String = lines.nth(self.line - 1).unwrap_or_default().into();
        let pointer = format!("{}👆 ----- This should not be here.  ", " ".repeat(col));

        write!(
            f,
            "{} `{}` on line {}.\n{}\n{}",
            self.message, self.lexeme, self.line, offending_line, pointer
        )
    }
}
