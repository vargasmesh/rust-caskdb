use crc::{Crc, CRC_16_IBM_SDLC};
use std::{fs::File, io::Write};

use crate::bitcask::Storage;

const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

pub struct Entry<'a> {
    key: &'a [u8],
    value: &'a [u8],
    timestamp: u64,
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

pub struct DiskStorage {
    active_data_file: std::fs::File,
}

impl DiskStorage {
    pub fn from_file(file: File) -> Self {
        Self {
            active_data_file: file,
        }
    }

    fn serialize_entry(&self, entry: &Entry) -> Vec<u8> {
        let key_size = entry.key.len();
        let value_size = entry.value.len();
        let capacity = 26 + key_size + value_size;
        let mut buf = vec![0; capacity];

        buf[2..10].copy_from_slice(&entry.timestamp.to_be_bytes());
        buf[10..18].copy_from_slice(&key_size.to_be_bytes());
        buf[18..26].copy_from_slice(&value_size.to_be_bytes());
        buf[26..26 + key_size].copy_from_slice(&entry.key);
        buf[26 + key_size..].copy_from_slice(&entry.value);

        let crc = X25.checksum(&buf[2..capacity]).to_be_bytes();

        buf[0..2].copy_from_slice(&crc);

        buf
    }
}

impl Storage for DiskStorage {
    fn write(&mut self, entry: &Entry) {
        let serialized = self.serialize_entry(entry);
        self.active_data_file.write_all(&serialized).unwrap();
        File::sync_data(&self.active_data_file).unwrap();
        File::sync_all(&self.active_data_file).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempfile;
    use time_macros::datetime;

    #[test]
    fn test_serialize_entry() {
        let entry = Entry {
            timestamp: datetime!(2021-01-01 00:00:00).assume_utc().unix_timestamp() as u64,
            key: "foo".as_bytes(),
            value: "bar".as_bytes(),
        };
        let file = tempfile().unwrap();
        let storage = DiskStorage::from_file(file);
        let serialized = storage.serialize_entry(&entry);
        assert_eq!(
            serialized,
            vec![
                0x99, 0xF0, // crc
                0x00, 0x00, 0x00, 0x00, 0x5f, 0xee, 0x66, 0x00, // 1609459200
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // 3
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // 3
                0x66, 0x6f, 0x6f, // foo
                0x62, 0x61, 0x72 // bar
            ]
        )
    }
}
