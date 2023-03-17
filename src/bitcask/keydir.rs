use std::collections::HashMap;

pub struct KeyDirEntry {
    pub file_id: String,
    pub value_size: usize,
    pub value_pos: u64,
    pub timestamp: u64,
}

pub type KeyDir<'a> = HashMap<&'a [u8], KeyDirEntry>;
