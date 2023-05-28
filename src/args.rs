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

    /// Skip setting terminal sequences
    #[arg(short, long)]
    pub skip_sequences: bool,

    /// Generates colors even if there is a cache version of it
    //ref: https://github.com/dylanaraps/pywal/issues/692
    //TODO short version will be `-o`, make sure we won't need that flag
    #[arg(short, long)]
    pub overwrite_cache: bool,
}
