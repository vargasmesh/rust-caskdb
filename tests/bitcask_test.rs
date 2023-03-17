use caskdb::{Bitcask, DiskStorage};
use tempfile::tempdir;

#[test]
fn get_hello_world() {
    let tempdir = tempdir().unwrap();
    let disk_storage = Box::new(DiskStorage::new(tempdir.path().to_str().unwrap()));
    let mut bitcask = Bitcask::new(disk_storage);
    bitcask.set(b"hello", b"world");
    assert_eq!(bitcask.get(b"hello").unwrap(), b"world");
}
