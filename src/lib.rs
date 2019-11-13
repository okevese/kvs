 
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

	pub fn get(&mut self, _key: String) -> Option<String> {
		self.entry.get(&_key).cloned()
	}

	pub fn remove(&mut self, _key: String) {
		self.entry.remove(&_key);
	}
}


