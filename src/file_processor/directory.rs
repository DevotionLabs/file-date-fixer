use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::Path;

use crate::logger::{error, warn};

use super::file::process_file;

pub fn process_dir(dir: &Path, recursive: bool) {
    let files = fs::read_dir(dir).unwrap_or_else(|err| {
        error(&format!(
            "Error: Unable to read specified directory: {}",
            err
        ));
        std::process::exit(1);
    });

    for entry in files {
        process_dir_entry(entry, recursive);
    }
}

fn process_dir_entry(entry: Result<DirEntry, Error>, recursive: bool) {
    if let Ok(entry) = entry {
        let file_path = entry.path();

        if file_path.is_dir() {
            handle_directory_entry(&file_path, recursive);
        } else {
            handle_file_entry(&file_path);
        }
    }
}

fn handle_directory_entry(dir_path: &Path, recursive: bool) {
    if recursive {
        process_dir(dir_path, recursive);
    } else {
        warn(&format!(
            "Skipping directory: {}. Use -r to process directories recursively.",
            dir_path.display()
        ));
    }
}

fn handle_file_entry(file_path: &Path) {
    if let Some(file_name) = extract_utf8_file_name(file_path) {
        process_file(file_path, file_name);
    } else {
        error(&format!(
            "Unable to extract file name from path: {}",
            file_path.display()
        ));
    }
}

fn extract_utf8_file_name(path: &Path) -> Option<&str> {
    path.file_name()?.to_str()
}
