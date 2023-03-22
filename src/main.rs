use std::str::from_utf8;

use caskdb::{Bitcask, DiskStorage};

fn main() {
    let storage = Box::new(DiskStorage::new("/tmp/bitcask"));
    let mut bitcask = Bitcask::new(storage);
    println!(
        "hello: {}",
        from_utf8(&bitcask.get(b"hello".to_vec()).unwrap()).unwrap()
    );
    println!(
        "hello2: {}",
        from_utf8(&bitcask.get(b"hello2".to_vec()).unwrap()).unwrap()
    );
}
