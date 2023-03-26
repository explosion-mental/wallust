//! Cli flags
//! * consider using the same flags as `pywal`, in order to be a drop-in replacement..
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Use this image
    pub file: PathBuf,
}
