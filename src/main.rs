#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod character_stream;
mod parser;
mod prettifier;
mod scanner;
mod token;
use parser::*;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut parser = Parser::new(&args[2]);

	parser.parse();
	println!("{}", parser.to_xhtml("X Formatted file"));
}
