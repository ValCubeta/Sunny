use std::collections::HashMap;
use crate::types::Value;

#[derive(Eq, PartialEq, Hash)]
pub struct Key(pub Box<str>);

impl From<&str> for Key {
	fn from(string: &str) -> Self {
		Key(Box::from(string))
	}
}

pub struct Dict(pub HashMap<Key, Value>);

impl<const N: usize> From<[(Key, Value); N]> for Dict {
	fn from(src: [(Key, Value); N]) -> Self {
		Dict(HashMap::from(src))
	}
}