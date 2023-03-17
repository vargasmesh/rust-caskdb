use std::str::from_utf8;

use caskdb::{Bitcask, DiskStorage};

fn main() {
    let storage = Box::new(DiskStorage::new("/tmp/bitcask"));
    let mut bitcask = Bitcask::new(storage);
    bitcask.set(b"hello", b"world");
    println!(
        "hello {}",
        from_utf8(&bitcask.get(b"hello").unwrap()).unwrap()
    );
}
