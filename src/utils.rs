// This file contains utility functions for file operations and data backup logic, such as reading files, writing files, and handling image/video data.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())
}

pub fn backup_media_files(source_dir: &str, backup_dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let backup_path = Path::new(backup_dir).join(file_name);
            fs::copy(&path, backup_path)?;
        }
    }
    Ok(())
}