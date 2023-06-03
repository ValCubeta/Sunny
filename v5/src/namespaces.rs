use std::collections::HashMap;
use crate::functions::parse_function;
use crate::types::Value;
use crate::errors::{unexpected, unexpected_token, debug};
use crate::words::parse_word;

pub struct Namespace {
	pub data: HashMap<String, Value>
}

#[allow(unused)]
pub fn parse_namespace(chars: &[char], i: &mut usize, is_main: bool) -> HashMap<String, Any> {
	let mut data = HashMap::new();
	while *i < chars.len() {
		let chr: char = chars[*i];
		match chr {
			'\n' | ' ' | '\t' | '\r' => {
				debug!("space");
			}
			'a'..='z' | '_' | 'A'..='Z' => {
				let word: String = parse_word(chars, i);
				match word.as_str() {
					"fun" => {
						let function = parse_function(chars, i);
						function.name;
					}
					_ => {
						unexpected_token(word);
					}
				}
			}
			_ => {
				unexpected(chr);
			}
		}
		*i += 1;
	}
	data
}