use anyhow::Result;
use std::{ffi::OsStr, fs, path::PathBuf};

pub fn list_md_files(root: &str) -> Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("md") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_owned());
        }
    }
    Ok(result)
}
