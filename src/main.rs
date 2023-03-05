use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use comrak::{markdown_to_html, ComrakOptions};

fn main() -> Result<()> {
    let files = get_files("./posts")?;
    for file in files {
        let contents = fs::read_to_string(file)?;
        let result = markdown_to_html(&contents, &ComrakOptions::default());
        println!("{result}")
    }
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
