use std::env;

pub fn working_directory() -> std::path::PathBuf {
    env::current_dir()
        .unwrap_or_else(|_| env::var("PWD").unwrap().into())
}

pub fn path_without_slash_suffix(path: &str) -> &str {
    if path.ends_with("/") {
        return &path[..path.len()-1];
    }
    return &path[..]
}
