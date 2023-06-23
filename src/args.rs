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
    //ref: <https://github.com/dylanaraps/pywal/issues/692>
    #[arg(short = 'c', long)]
    pub overwrite_cache: bool,

    /// Don't cache the results
    #[arg(short, long)]
    pub no_cache: bool,

    /// Use PATH as the config directory
    #[arg(short = 'C', long, value_name = "PATH")]
    pub config_path: Option<PathBuf>,
}
