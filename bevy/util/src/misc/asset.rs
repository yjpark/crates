use std::path::PathBuf;

fn check_assets_path(root: PathBuf) -> Option<PathBuf> {
    let mut path = root.clone();
    path.push("assets");
    if !path.exists() && path.is_dir() {
        println!(
            "check_assets_path() not exist: {:?} -> {:?}",
            root, path
        );
    } else if !path.is_dir() {
        println!(
            "check_assets_path() is not dir: {:?} -> {:?}",
            root, path
        );
    } else {
        return Some(path);
    }
    None
}

pub fn get_folder() -> Option<PathBuf> {
    let mut path = None;
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_folder) = exe_path.parent() {
            path = check_assets_path(exe_folder.to_path_buf());
        }
    }
    if path.is_none() {
        if let Ok(cwd) = std::env::current_dir() {
            path = check_assets_path(cwd.to_path_buf());
        }
    }
    path
}

fn _get_path(folder: PathBuf, name: &str, extension: &str) -> Option<PathBuf> {
    let mut path = folder.clone();
    path.push(name);
    path.set_extension(extension);
    if path.exists() {
        Some(path)
    } else {
        println!(
            "_get_path() not exist: {:?} {}.{} -> {:?}",
            folder, name, extension, path
        );
        None
    }
}

pub fn get_path(name: &str, extension: &str) -> Option<PathBuf> {
    match get_folder() {
        Some(folder) => _get_path(folder, name, extension),
        None => None,
    }
}
