use std::str::from_utf8;

use caskdb::{Bitcask, DiskStorage};

fn main() {
    let storage = Box::new(DiskStorage::new("/tmp/bitcask"));
    let mut bitcask = Bitcask::new(storage);
    bitcask.set(b"hello".to_vec(), b"world".to_vec());
    println!(
        "hello: {}",
        from_utf8(&bitcask.get(b"hello".to_vec()).unwrap()).unwrap()
    );
}
