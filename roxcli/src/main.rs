use clap::Parser;
use miette::{Context, IntoDiagnostic, Result};
use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::{
    fs::{self, File},
    io::Read,
};

mod error;

#[derive(Debug, Parser)]
struct Args {
    source: Option<String>,
}

fn main() -> Result<()> {
    let cli = Args::parse();

    match cli.source {
        Some(source) => run_file(&source),
        None => run_prompt(),
    }
}

fn run_file(source: &str) -> Result<()> {
    match (File::open(source), fs::metadata(source)) {
        (Ok(mut f), meta) => {
            let mut buf = match meta {
                Ok(meta) => Vec::with_capacity(meta.len() as usize),
                Err(e) => {
                    error::MetadataWarning::render(source.to_string(), e);
                    Vec::new()
                }
            };
            f.read_to_end(&mut buf)
                .into_diagnostic()
                .with_context(|| source.to_string())?;
            let s = unsafe { String::from_utf8_unchecked(buf) };
            if let Err(e) = roxc::run(&s).map(|_| ()) {
                error(e);
                std::process::exit(69);
            }
            Ok(())
        }
        (Err(e), _) => Err(e)
            .into_diagnostic()
            .with_context(|| format!("something went wrong accessing \"{source}\"")),
    }
}

fn run_prompt() -> Result<()> {
    let mut rl = Editor::<()>::new().into_diagnostic()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = roxc::run(&line).map_err(error);
            }
            Err(ReadlineError::Interrupted) => {
                println!("⚠️: CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("👋🏾 BYE 👋 BYE 👋🏻");
                break;
            }
            Err(err) => {
                println!("🚨: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn error(error: roxc::ErrorReport) {
    report(error.line, "", &error.message)
}

fn report(line: usize, r#where: &str, message: &str) {
    println!("[line {line}] Error{where}: {message}");
}
