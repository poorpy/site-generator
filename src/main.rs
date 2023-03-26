mod cli;
mod css;
mod generator;
mod watcher;

use std::path::PathBuf;
use std::process::exit;
use std::time::Duration;
use std::{fs, path::Path};

use anyhow::{Context, Result};
use clap::Parser;
use crossbeam::channel::Sender;
use log::{debug, error};
use notify::{INotifyWatcher, RecursiveMode};
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
            css::generate_css("./tailwind/input.css", "./output/output.css")?;
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
            let (s1, r1) = crossbeam::channel::unbounded();

            let mut notes_debouncer = new_debounced_watcher(s1)?;
            notes_debouncer
                .watcher()
                .watch(&args.notes, RecursiveMode::Recursive)?;

            let (s2, r2) = crossbeam::channel::unbounded();
            let mut templates_debouncer = new_debounced_watcher(s2)?;
            templates_debouncer
                .watcher()
                .watch(Path::new("templates"), RecursiveMode::Recursive)?;

            let mut watcher = watcher::Watcher::new(
                Generator::new(args.notes, args.output, "templates")
                    .context("failed to create new generator")?,
                r1,
                r2,
            );

            watcher.watch();
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

fn new_debounced_watcher(writer: Sender<PathBuf>) -> Result<Debouncer<INotifyWatcher>> {
    Ok(new_debouncer(
        Duration::from_secs(1),
        None,
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                debug!("events: {:#?}", events);
                for event in events {
                    writer.send(event.path).unwrap();
                }
            }
            Err(e) => error!("watch error: {:?}", e),
        },
    )?)
}
