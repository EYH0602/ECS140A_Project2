#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod token;
mod character_stream;
mod scanner;
mod parser;
use parser::*;


use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut parser = Parser::new(&args[2]);

	parser.print_lex_results();
}
