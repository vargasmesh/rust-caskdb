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
