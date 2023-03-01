use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    get_files("./posts")?;
    Ok(())
}

fn get_files(post_directory: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let files = fs::read_dir(post_directory).context("failed to open posts directory")?;

    Ok(files
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_file())
        .map(|f| f.path())
        .filter(|f| {
            if let Some(ext) = f.extension() {
                return ext == "md";
            }
            false
        })
        .collect())
}
