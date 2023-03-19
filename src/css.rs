use std::path::Path;
use std::process::Command;
use std::str;

use anyhow::Result;
use log::{error, info};

pub fn tailwind_watcher() -> Command {
    let mut command = Command::new("tailwind");
    command.args([
        "-i",
        "./tailwind/input.css",
        "-o",
        "./output/output.css",
        "--watch",
    ]);

    command
}

pub fn generate_css(input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
    info!("generating css with tailwind ...");
    let (input, output) = (input.as_ref(), output.as_ref());

    let mut command: Command = Command::new("tailwind");
    command.args(["-i", &format!("{input:?}"), "-o", &format!("{output:?}")]);

    let output = command.output()?;

    if !output.status.success() {
        error!(
            "command {command:?} failed with error:\n{}",
            str::from_utf8(&output.stderr)?
        );
    }

    info!("done");

    Ok(())
}
