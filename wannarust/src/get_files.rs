use std::path::PathBuf;
use walkdir::WalkDir;

fn is_valid_extension(ext: &str) -> bool {
    let invalid_extensions = [
        "der", "key", "exe", "dll", "sys", "ini", "lnk", "bat", "cmd", "com", "scr", "pif", "wnrs",
    ];
    !invalid_extensions.contains(&ext)
}

pub fn get_target_files(path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if is_valid_extension(ext.to_str().unwrap_or("")) {
                    files.push(path.to_path_buf());
                }
            }
        }
    }

    files
}

pub fn get_infected_files(path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "wnrs" {
                    files.push(path.to_path_buf());
                }
            }
        }
    }

    files
}
