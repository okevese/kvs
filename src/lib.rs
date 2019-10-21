 

#[derive(PartialEq)]
#[derive(Debug)]
pub struct KvStore {
	key: String,
	value: String,
}

impl KvStore {
	pub fn new() -> KvStore {
		KvStore {
			key: "".to_owned(),
			value: "".to_owned(),
		}
	}
	
	pub fn set(&mut self, _key: String, _value: String)  {
		self.key = _key;
		self.value = _value;
	}

	pub fn get(&self, _key: String) -> Option<String> {
		unimplemented!();
	}

	pub fn remove(&self, _key: String) {
		unimplemented!();
	}
}


