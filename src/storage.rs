pub struct Entry<'a> {
    timestamp: i64,
    key: &'a [u8],
    value: &'a [u8],
}

pub struct DiskStorage {}

impl DiskStorage {
    pub fn new() -> Self {
        DiskStorage {}
    }

    fn serialize_entry(&self, entry: &Entry) -> Vec<u8> {
        let key_size = entry.key.len();
        let value_size = entry.value.len();
        let mut buf = Vec::with_capacity(24 + key_size + value_size);
        buf.extend_from_slice(&entry.timestamp.to_be_bytes());
        buf.extend_from_slice(&key_size.to_be_bytes());
        buf.extend_from_slice(&value_size.to_be_bytes());
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
            key: "foo".as_bytes(),
            value: "bar".as_bytes(),
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
