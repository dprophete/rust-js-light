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

fn main() {
    let args = MainArgs::parse();

    // expr
    if let Some(expr) = &args.expr {
        run_prg(expr);
        process::exit(0)
    } else
    // file
    if let Some(file) = &args.file {
        match fs::read_to_string(file) {
            Ok(content) => {
                run_prg(&content);
                process::exit(0)
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", file, e);
                process::exit(1)
            }
        }
    }

    MainArgs::command().print_help().unwrap();
    process::exit(0)
}

fn run_prg(content: &str) {
    let prg = parser::parse_prg(&content).expect("parsing errro");
    println!("parsed prg:\n{}", prg);

    println!("executing prg");
    let mut runner = runner::Runner::new();
    runner.run_prg(&prg);
    runner.print_vars();
}
