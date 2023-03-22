use crc::{Crc, CRC_16_IBM_SDLC};
use std::{
    fs::{canonicalize, create_dir_all, read_dir, File},
    io::{Read, Seek, SeekFrom, Write},
};

use crate::bitcask::{Entry, KeyDir, Storage, ValuePosition};

const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

struct DataFile {
    file: File,
    directory: String,
    filename: String,
}

pub struct DiskStorage {
    active_data_file: DataFile,
    current_position: usize,
}

impl DiskStorage {
    pub fn new(directory: &str) -> Self {
        let filename = DiskStorage::create_active_filename();
        create_dir_all(directory).unwrap();
        let file = File::create(format!("{}/{}", directory, filename,)).unwrap();
        Self {
            current_position: 0,
            active_data_file: DataFile {
                file,
                directory: directory.to_string(),
                filename,
            },
        }
    }

    fn create_active_filename() -> String {
        cuid2::create_id()
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

struct DiskValuePosition {
    file_id: String,
    value_size: usize,
    value_position: usize,
}

impl ValuePosition for DiskValuePosition {
    fn get_value(&self) -> Vec<u8> {
        let mut file = File::open(&self.file_id).unwrap();
        let mut buf = vec![0; self.value_size];
        file.seek(std::io::SeekFrom::Start(self.value_position as u64))
            .unwrap();
        file.read_exact(&mut buf).unwrap();

        buf
    }
}

impl Storage for DiskStorage {
    fn write(&mut self, entry: &Entry) -> Box<dyn ValuePosition> {
        let serialized = self.serialize_entry(entry);

        self.active_data_file.file.write_all(&serialized).unwrap();
        File::sync_data(&self.active_data_file.file).unwrap();
        File::sync_all(&self.active_data_file.file).unwrap();

        let value_position = self.current_position + 26 + entry.key.len();
        self.current_position += serialized.len();

        Box::new(DiskValuePosition {
            file_id: format!(
                "{}/{}",
                self.active_data_file.directory, self.active_data_file.filename
            ),
            value_size: entry.value.len(),
            value_position,
        })
    }

    fn load_keydir(&self, keydir: &mut dyn KeyDir) {
        let mut files = read_dir(&self.active_data_file.directory)
            .unwrap()
            .map(|f| canonicalize(f.unwrap().path()).unwrap())
            .take_while(|path| path.is_file())
            .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        files.sort();
        files.iter().for_each(|path| {
            let mut file =
                File::open(format!("{}/{}", self.active_data_file.directory, path)).unwrap();

            let stream_size = file.metadata().unwrap().len();

            let mut offset = 10;
            while offset < stream_size {
                match file.seek(SeekFrom::Current(offset as i64)) {
                    Ok(_) => {
                        let mut key_size_buf = [0; 8];
                        file.read_exact(&mut key_size_buf).unwrap();
                        let key_size = u64::from_be_bytes(key_size_buf);
                        let mut value_size_buf = [0; 8];
                        file.read_exact(&mut value_size_buf).unwrap();
                        let value_size = u64::from_be_bytes(value_size_buf);
                        let mut key_buffer = vec![0; key_size as usize];
                        file.read_exact(&mut key_buffer).unwrap();
                        let value_position = offset + 16 + key_size;
                        keydir.set(
                            key_buffer,
                            Box::new(DiskValuePosition {
                                file_id: format!("{}/{}", self.active_data_file.directory, path),
                                value_size: value_size as usize,
                                value_position: value_position as usize,
                            }),
                        );
                        offset += 16 + key_size + value_size;
                    }
                    Err(_) => return,
                };
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use time_macros::datetime;

    #[test]
    fn test_serialize_entry() {
        let entry = Entry {
            timestamp: datetime!(2021-01-01 00:00:00).assume_utc().unix_timestamp() as u64,
            key: b"foo".to_vec(),
            value: b"bar".to_vec(),
        };
        let directory = tempdir().unwrap();
        let storage = DiskStorage::new(directory.path().to_str().unwrap());
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
