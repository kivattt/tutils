use std::env;

pub fn working_directory() -> std::path::PathBuf {
    env::current_dir()
        .unwrap_or_else(|_| env::var("PWD").unwrap().into())
}
