 
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct KvStore {
	entry: HashMap<String, String>,
}

impl KvStore {
	pub fn new() -> KvStore {
		KvStore {
			entry: HashMap::new(),
		}
	}
	
	pub fn set(&mut self, _key: String, _value: String)  {
		self.entry.insert(_key, _value);
	}

	pub fn get(&self, _key: String) -> Option<String> {
		self.entry.get(_key)
	}

	pub fn remove(&self, _key: String) {
		unimplemented!();
	}
}


