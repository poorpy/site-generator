mod generator;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use comrak::{markdown_to_html, ComrakOptions};

fn main() -> Result<()> {
    let files = get_posts("./posts")?;
    for file in files {
        let result = read_to_html(file)?;
        println!("{result}")
    }
    Ok(())
}

// fn write_as_html(path: impl AsRef<Path>, )

fn read_to_html(path: impl AsRef<Path>) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(markdown_to_html(&contents, &ComrakOptions::default()))
}

fn get_posts(post_directory: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
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
