use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    // Path to a folder with notes
    #[arg(short, long, value_name = "NOTES_DIR", default_value = "notes")]
    pub notes: PathBuf,

    // Path to a output folder
    #[arg(short, long, value_name = "OUTPUT_DIR", default_value = "output")]
    pub output: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate html files from md notes
    Generate,
}
