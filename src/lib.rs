use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]

pub struct KvStore {
    entry: HashMap<String, String>,
}

impl KvStore {
    // Create a new kvStore hashmap
    pub fn new() -> KvStore {
        KvStore {
            entry: HashMap::new(),
        }
    }
    // Inserts `key` in the  KvStore hashmap
    pub fn set(&mut self, _key: String, _value: String) {
        self.entry.insert(_key, _value);
    }

    // Gets `key` from the KvStore hashmap
    pub fn get(&mut self, _key: String) -> Option<String> {
        self.entry.get(&_key).cloned()
    }

    // Removes `key` from the kvStore hashmap
    pub fn remove(&mut self, _key: String) {
        self.entry.remove(&_key);
    }
}
