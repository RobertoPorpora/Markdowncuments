use std::fs;
use std::path::{Path, PathBuf};

pub fn find_all(extension: &str, folder: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    visit_dirs(folder, extension, recursive, &mut files);

    files
}

fn visit_dirs(dir: &Path, extension: &str, recursive: bool, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();

            if recursive && path.is_dir() {
                visit_dirs(&path, extension, recursive, files);
            } else if path.is_file() && is_matching_extension(&path, extension) {
                files.push(path);
            }
        }
    }
}

fn is_matching_extension(path: &Path, extension: &str) -> bool {
    if let Some(ext) = path.extension() {
        return ext == extension;
    }
    false
}
