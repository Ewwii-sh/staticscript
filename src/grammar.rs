use pest_derive::Parser;

// This tells Rust to use `grammar.pest` as the grammar
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DslParser;
