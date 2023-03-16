mod bitcask;
mod pkg;
mod storage;

fn main() {
    let storage = Box::new(storage::DiskStorage::new("test.db".into()));
    let mut bitcask = bitcask::Bitcask::new(storage);
    bitcask.set(b"hello", b"world");
}
