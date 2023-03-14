pub struct Entry {
    timestamp: i64,
    key: Vec<u8>,
    value: Vec<u8>,
}

pub struct DiskStorage {}

impl DiskStorage {
    pub fn new() -> Self {
        DiskStorage {}
    }

    fn serialize_entry(&self, entry: &Entry) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&entry.timestamp.to_be_bytes());
        buf.extend_from_slice(&entry.key.len().to_be_bytes());
        buf.extend_from_slice(&entry.value.len().to_be_bytes());
        buf.extend_from_slice(&entry.key);
        buf.extend_from_slice(&entry.value);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time_macros::datetime;

    #[test]
    fn test_serialize_entry() {
        let entry = Entry {
            timestamp: datetime!(2021-01-01 00:00:00).assume_utc().unix_timestamp(),
            key: "foo".as_bytes().to_vec(),
            value: "bar".as_bytes().to_vec(),
        };
        let storage = DiskStorage::new();
        let serialized = storage.serialize_entry(&entry);
        assert_eq!(
            serialized,
            vec![
                0x00, 0x00, 0x00, 0x00, 0x5f, 0xee, 0x66, 0x00, // 1609459200
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // 3
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // 3
                0x66, 0x6f, 0x6f, // foo
                0x62, 0x61, 0x72 // bar
            ]
        )
    }
}
