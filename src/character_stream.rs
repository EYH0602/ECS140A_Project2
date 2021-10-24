use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct CharStream {
	contents: String,
}

impl CharStream {
	pub fn new(f: &str) -> CharStream {
		match File::open(f) {
			Ok(mut file) => {
				let mut contents = String::new();
				file.read_to_string(&mut contents).unwrap();
				CharStream { contents: contents }
			}
			Err(_) => {
				panic!("Error opening file {}", f);
			}
		}
	}

	pub fn get_contents(&self) -> &String {
		&self.contents
	}

	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
		!self.contents.is_empty()
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available.
	pub fn peek_next_char(&self) -> Option<char> {
		self.peek_ahead_char(0)
	}

	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	pub fn peek_ahead_char(&self, k: i32) -> Option<char> {
		match k < 0 {
			true => None,
			false => {
				let ch = self.contents.chars().nth(k as usize).unwrap();
				Some(ch)
			}
		}
	}

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {
		match self.peek_next_char() {
			None => None,
			Some(ch) => {
				// consume the first char
				let mut chars = self.contents.chars();
				chars.next();
				chars.next_back();
				self.contents = String::from(chars.as_str());
				Some(ch)
			}
		}
	}
}
