#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod character_stream;
use character_stream::*;

mod scanner;
use scanner::*;

mod token;
use token::*;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut scanner = Scanner::new(&args[2]);

	loop {
		match scanner.get_next_token() {
			Some(token) => {
				println!("text: {}", token.get_text());
				println!("token type: {}", token.get_type().as_str());
				println!("line number: {}", token.get_line_number());
				println!("char position: {}", token.get_char_pos());
				println!("=======================================");
			}
			None => break,
		}
	}
	
	// let mut cs = CharStream::new(&args[2]);
	// let mut i = 0;
	// while cs.more_available() {
	// 	match cs.get_next_char() {
	// 		None => continue,
	// 		Some(ch) => {
	// 			println!("{}: {}", i, ch);
	// 			if ch == '\n' {
	// 				println!("hit next line");
	// 			}
	// 		}
	// 	}
	// 	i += 1;
	// }
}
