use caskdb::{Bitcask, DiskStorage};
use tempfile::tempdir;

#[test]
fn test_disk_storage_set_and_get() {
    let tempdir = tempdir().unwrap();
    let disk_storage = Box::new(DiskStorage::new(tempdir.path().to_str().unwrap()));
    let mut bitcask = Bitcask::new(disk_storage);
    bitcask.set(b"hello", b"world");
    assert_eq!(bitcask.get(b"hello").unwrap(), b"world");
}
