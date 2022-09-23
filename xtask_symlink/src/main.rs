use std::env;
use std::fs;
use std::os::unix::fs as unix_fs;

fn main() {
    let dir = env::current_dir().unwrap();
    let hns = &format!("{}/target/release/hns", dir.display());
    let hnsnn = &format!("{}/target/release/hns+", dir.display());
    let hxs = &format!("{}/target/release/hxs", dir.display());
    fs::remove_file(hnsnn).unwrap();
    fs::remove_file(hxs).unwrap();
    unix_fs::symlink(hns, hnsnn).unwrap();
    unix_fs::symlink(hns, hxs).unwrap();
}
