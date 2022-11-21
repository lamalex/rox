use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum ScannerError {
    UnrecognizedToken(ScannerErrorMeta),
    UnterminatedString(ScannerErrorMeta),
    Deserialization(ScannerErrorMeta),
}

#[derive(Debug, Clone)]
pub struct ScannerErrorMeta {
    pub lexeme: Option<char>,
    pub line: usize,
    pub position: usize,
    pub source: String,
}

impl error::Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (meta, top_level_message, pointer_message) = match self {
            Self::UnrecognizedToken(m) => (m, "Unrecognized token", "This should not be here"),
            Self::UnterminatedString(m) => (
                m,
                "Unterminated open quote",
                "This quotation mark is missing its partner",
            ),
            ScannerError::Deserialization(m) => (
                m,
                "Failed value conversion",
                "This could not be turned into a native value",
            ),
        };

        let mut lines = meta.source.split('\n');
        let offset = lines.clone().take(meta.line - 1).collect::<String>().len();
        let col = meta.position - offset - meta.line;

        let offending_line: String = lines.nth(meta.line - 1).unwrap_or_default().into();

        let pointer = format!("{}👆 ----- {}.  ", " ".repeat(col), pointer_message);

        write!(
            f,
            "ERROR: {}{} on line {}.\n{}\n{}",
            top_level_message,
            meta.lexeme.map(|c| format!(" `{}`", c)).unwrap_or_default(),
            meta.line,
            offending_line,
            pointer
        )
    }
}
