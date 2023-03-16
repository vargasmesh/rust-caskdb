use std::time;

use crate::{pkg::now, storage::Entry};

pub trait Storage {
    fn write(&mut self, entry: &Entry);
}

pub struct Bitcask {
    storage: Box<dyn Storage>,
}

impl Bitcask {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        let entry = Entry::new(key, value, now());
        self.storage.write(&entry);
    }
}
