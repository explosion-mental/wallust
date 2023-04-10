//! Cli flags
//! * consider using the same flags as `pywal`, in order to be a drop-in replacement..
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to an image file to use
    pub file: PathBuf,

    /// Don't print anything
    #[arg(short, long)]
    pub quiet: bool,
}
