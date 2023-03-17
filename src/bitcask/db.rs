use std::collections::HashMap;

use crate::pkg::now;

pub trait ValuePosition {
    fn get_value(&self) -> Vec<u8>;
}

pub trait Storage {
    fn write(&mut self, entry: &Entry) -> Box<dyn ValuePosition>;
}

pub struct Entry<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
    pub timestamp: u64,
}

impl<'a> Entry<'a> {
    pub fn new(key: &'a [u8], value: &'a [u8], timestamp: u64) -> Self {
        Self {
            key,
            value,
            timestamp,
        }
    }
}

pub struct Bitcask<'a> {
    storage: Box<dyn Storage>,
    keydir: HashMap<&'a [u8], Box<dyn ValuePosition>>,
}

impl<'a> Bitcask<'a> {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self {
            storage,
            keydir: HashMap::new(),
        }
    }

    pub fn get(&self, key: &'a [u8]) -> Option<Vec<u8>> {
        self.keydir
            .get(key)
            .map(|value_position| value_position.get_value())
    }

    pub fn set(&mut self, key: &'a [u8], value: &[u8]) {
        let entry = Entry::new(key, value, now());
        let value_position = self.storage.write(&entry);
        self.keydir.insert(key, value_position);
    }
}
