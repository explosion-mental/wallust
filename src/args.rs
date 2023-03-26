//! Cli flags
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Use this image
    pub file: PathBuf,
}
