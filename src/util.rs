use std::{ffi::OsStr, path::Path};

pub fn load_bytes(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut path = std::env::current_dir()?;
    path.push("res");
    path.push(file_name);

    let data = std::fs::read(path)?;
    Ok(data)
}

pub fn get_extension(file_name: &str) -> Option<&str> {
    Path::new(file_name)
        .extension()
        .and_then(OsStr::to_str)
}
