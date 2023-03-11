mod cli;
mod generator;

use std::{fs, path::Path};

use generator::Generator;

use anyhow::{Context, Result};
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();

    let args = cli::Args::parse();

    debug!("{args:?}");

    create_dir(&args.notes)?;
    create_dir(&args.output)?;

    Generator::new(args.notes, args.output, "templates")
        .context("failed to create new generator")?
        .render()
        .context("failed to generate posts")?;

    Ok(())
}

fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    if !path.as_ref().is_dir() {
        return Ok(fs::create_dir_all(path)?);
    }
    Ok(())
}
