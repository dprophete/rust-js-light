use clap::{CommandFactory, Parser as _};

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "parser/lang.pest"]
pub struct LangParser;

#[derive(clap::Parser, Debug)]
#[command(about = "A tiny js-like interpreter", long_about = None)]
pub struct MainArgs {
    #[clap(long)]
    file: Option<String>,
    #[clap(long)]
    expr: Option<String>,
}

use std::{fs, process};

mod parser;
mod runner;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse error {0:?}")]
    ParseError(#[from] parser::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args = MainArgs::parse();

    // expr
    if let Some(expr) = &args.expr {
        run_prg(&expr).unwrap_or_else(|e| {
            eprintln!("Error running expr: {}", e);
            process::exit(1)
        });
        process::exit(0)
    }

    // file
    if let Some(file) = &args.file {
        let content = fs::read_to_string(file).unwrap_or_else(|e| {
            eprintln!("Error reading file {}: {}", file, e);
            process::exit(1)
        });
        run_prg(&content).unwrap_or_else(|e| {
            eprintln!("Error running prg: {}", e);
            process::exit(1)
        });
        process::exit(0)
    }

    MainArgs::command().print_help().unwrap();
    process::exit(0)
}

fn run_prg(content: &str) -> Result<()> {
    let prg = parser::parse_prg(&content)?;
    println!("parsed prg:\n{}", prg);

    println!("executing prg");
    let mut runner = runner::Runner::new();
    runner.run_prg(&prg);
    runner.print_vars();
    Ok(())
}
