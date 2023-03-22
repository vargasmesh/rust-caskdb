use std::{fs::File, io::Write};

use caskdb::{Bitcask, DiskStorage};
use tempfile::tempdir;

#[test]
fn test_disk_storage_set_and_get() {
    let tempdir = tempdir().unwrap();
    let disk_storage = Box::new(DiskStorage::new(tempdir.path().to_str().unwrap()));
    let mut bitcask = Bitcask::new(disk_storage);
    bitcask.set(b"hello".to_vec(), b"world".to_vec());
    assert_eq!(bitcask.get(b"hello".to_vec()).unwrap(), b"world");
}

#[test]
fn test_disk_storage_load_dir() {
    let tempdir = tempdir().unwrap();
    let previous_data_file_content = vec![
        0x99, 0xF0, // crc
        0x00, 0x00, 0x00, 0x00, 0x5f, 0xee, 0x66, 0x00, // 1609459200
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // 3
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // 3
        0x66, 0x6f, 0x6f, // foo
        0x62, 0x61, 0x72, // bar
    ];
    let mut previous_data_file =
        File::create(tempdir.path().join("s9l0ncbr08iu14hlcxixbb66")).unwrap();
    previous_data_file
        .write_all(&previous_data_file_content)
        .unwrap();
    let disk_storage = Box::new(DiskStorage::new(tempdir.path().to_str().unwrap()));
    let mut bitcask = Bitcask::new(disk_storage);

    bitcask.set(b"hello".to_vec(), b"world".to_vec());
    assert_eq!(bitcask.get(b"hello".to_vec()).unwrap(), b"world");
    assert_eq!(bitcask.get(b"foo".to_vec()).unwrap(), b"bar");
}
