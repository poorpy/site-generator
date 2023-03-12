mod cli;
mod css;
mod generator;

use std::process::exit;
use std::time::Duration;
use std::{fs, path::Path};

use anyhow::{Context, Result};
use clap::Parser;
use cli::Args;
use log::{debug, error, info};
use notify::INotifyWatcher;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, Debouncer};

use crate::cli::Commands;
use crate::generator::Generator;

fn main() -> Result<()> {
    env_logger::init();

    let args = cli::Args::parse();

    debug!("{args:?}");

    create_dir(&args.notes)?;
    create_dir(&args.output)?;

    match args.command {
        Commands::Generate => {
            Generator::new(args.notes, args.output, "templates")
                .context("failed to create new generator")?
                .render()
                .context("failed to generate posts")?;
        }
        Commands::Watch => {
            let mut tailwind = css::tailwind_watcher().spawn()?;
            ctrlc::set_handler(move || {
                tailwind.kill().unwrap();
                exit(0)
            })?;
            let mut watcher = new_watcher(&args)?;
            loop {
                watcher
                    .watcher()
                    .watch(Path::new(&args.notes), RecursiveMode::Recursive)?;
            }
        }
    }

    Ok(())
}

fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    if !path.as_ref().is_dir() {
        return Ok(fs::create_dir_all(path)?);
    }
    Ok(())
}

fn new_watcher(args: &Args) -> Result<Debouncer<INotifyWatcher>> {
    let gen = Generator::new(&args.notes, &args.output, "templates")?;
    Ok(new_debouncer(
        Duration::from_secs(1),
        None,
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                info!("events: {:#?}", events);
                for event in events {
                    info! {"{:?}", event.path}
                    if let Err(e) = gen.render_path(event.path) {
                        error!("render error: {:?}", e);
                    }
                }
            }
            Err(e) => error!("watch error: {:?}", e),
        },
    )?)
}
