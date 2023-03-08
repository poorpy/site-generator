mod cli;
mod generator;

use generator::Generator;

use anyhow::{Context, Result};
use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    env_logger::init();

    let args = cli::Args::parse();

    debug!("{args:?}");

    Generator::new(args.notes, args.output)
        .context("failed to create new generator")?
        .render()
        .context("failed to generate posts")?;

    Ok(())
}
