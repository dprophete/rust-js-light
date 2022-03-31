extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lang.pest"]
pub struct LangParser;

use std::fs;

mod parser;
mod runner;

fn main() {
    let file_content = fs::read_to_string("resources/ex1.js_new").expect("cannot read file");
    let prg = parser::parse_prg(&file_content).expect("cannot parse file");
    println!("parsed prg:\n{}", prg);

    println!("executing prg");
    let mut runner = runner::Runner::new();
    runner.run_prg(&prg);
    runner.print_vars()
}
