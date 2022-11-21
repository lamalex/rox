use scanner::Scanner;

mod scanner;
mod token;

#[derive(Debug, Clone)]
pub struct ErrorReport {
    pub line: usize,
    pub source: String,
    pub message: String,
}

pub fn run(source: &str) -> Result<String, ErrorReport> {
    let scanner = Scanner::new(source);
    for token in scanner {
        println!("{token:?}");
        if let Err(e) = token {
            println!("{}", e);
        }
    }

    Ok("".into())
}
