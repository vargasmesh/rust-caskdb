use crate::pkg::now;

use super::keydir::{HashMapKeyDir, KeyDir};

pub trait ValuePosition {
    fn get_value(&self) -> Vec<u8>;
}

pub trait Storage {
    fn write(&mut self, entry: &Entry) -> Box<dyn ValuePosition>;
    fn load_keydir(&self, keydir: &mut dyn KeyDir);
}

pub struct Entry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: u64,
}

impl Entry {
    pub fn new(key: Vec<u8>, value: Vec<u8>, timestamp: u64) -> Self {
        Self {
            key,
            value,
            timestamp,
        }
    }
}

pub struct Bitcask {
    storage: Box<dyn Storage>,
    keydir: Box<dyn KeyDir>,
}

impl Bitcask {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        let mut keydir = Box::new(HashMapKeyDir::new());
        storage.load_keydir(keydir.as_mut());
        Self { storage, keydir }
    }

    pub fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.keydir.get(key)
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let entry = Entry::new(key, value, now());
        let value_position = self.storage.write(&entry);
        self.keydir.set(entry.key, value_position);
    }
}
