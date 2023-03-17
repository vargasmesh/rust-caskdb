mod bitcask;
mod pkg;
mod storage;

use std::str::from_utf8;

fn main() {
    let storage = Box::new(storage::DiskStorage::new("/tmp/bitcask"));
    let mut bitcask = bitcask::Bitcask::new(storage);
    bitcask.set(b"hello", b"world");
    println!(
        "hello {}",
        from_utf8(&bitcask.get(b"hello").unwrap()).unwrap()
    );
}
