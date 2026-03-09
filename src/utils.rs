use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
