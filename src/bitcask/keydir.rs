use std::collections::HashMap;

use super::ValuePosition;

pub trait KeyDir {
    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>>;
    fn set(&mut self, key: Vec<u8>, value: Box<dyn ValuePosition>);
}

pub struct HashMapKeyDir {
    keydir: HashMap<Vec<u8>, Box<dyn ValuePosition>>,
}

impl HashMapKeyDir {
    pub fn new() -> Self {
        Self {
            keydir: HashMap::new(),
        }
    }
}

impl KeyDir for HashMapKeyDir {
    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.keydir
            .get(&key)
            .map(|value_position| value_position.get_value())
    }

    fn set(&mut self, key: Vec<u8>, value: Box<dyn ValuePosition>) {
        self.keydir.insert(key, value);
    }
}
