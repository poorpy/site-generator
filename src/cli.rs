use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    // Path to a folder with notes
    #[arg(short, long, value_name = "NOTES_DIR")]
    pub notes: PathBuf,

    // Path to a output folder
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    pub output: PathBuf,
}
