use std::{ffi::OsStr, path::Path};

pub fn load_bytes(file_name: &str) -> Vec<u8> {
    let mut path = std::env::current_dir()
        .expect("Current working directory is not valid");

    path.push("res");
    path.push(file_name);

    std::fs::read(path)
        .expect("Could not read given path!")
}

pub fn get_extension(file_name: &str) -> Option<&str> {
    Path::new(file_name)
        .extension()
        .and_then(OsStr::to_str)
}
