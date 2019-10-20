 

#[derive(PartialEq)]
#[derive(Debug)]
pub struct KvStore {
	key: String,
	value: String,
}

impl KvStore {
	pub fn new(key: String, value: String) -> KvStore {
		KvStore {
			key: key,
			value: value,
		}
	}
	
	pub fn set(&mut self, _key: String, _value: String)  {
		unimplemented!();
	}

	pub fn get(&self, _key: String) -> Option<String> {
		unimplemented!();
	}

	pub fn remove(&self, _key: String) {
		unimplemented!();
	}
}


