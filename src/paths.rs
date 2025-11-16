use std::{env, fs, path::PathBuf, sync::Mutex};

use is_executable::IsExecutable;
use once_cell::sync::Lazy;

pub static PATHS: Lazy<Mutex<Vec<(String, PathBuf)>>> = Lazy::new(|| Mutex::new(vec![]));

pub fn search_path(name: &str) -> Option<PathBuf> {
    PATHS.lock().unwrap().iter().find_map(|file| {
        if file.0 == name {
            Some(file.1.clone())
        } else {
            None
        }
    })
}

pub fn precompute_path() {
    let dirs = env::var_os("PATH")
        .into_iter()
        .flat_map(|os| env::split_paths(&os).collect::<Vec<_>>());

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            entries
                .filter_map(Result::ok)
                .filter(|file| file.path().is_executable())
                .for_each(|f| {
                    PATHS
                        .lock()
                        .unwrap()
                        .push((f.file_name().to_string_lossy().to_string(), f.path()))
                });
        }
    }
}
