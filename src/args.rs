//! Cli flags
//! * consider using the same flags as `pywal`, in order to be a drop-in replacement..
//! TODO make sure this works properly with `clap_completions`, currently it doesn't.

use std::path::PathBuf;

use crate::{
    backends::Backend,
    colorspaces::ColorSpaces,
    filters::Filters,
    themes::Schemes,
    themes::COLS_KEY,
};

use clap::Parser;

/// Overall cli type for clap
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(subcommand_negates_reqs(true))]
#[command(args_conflicts_with_subcommands(true))]
pub struct Cli {
    #[clap(flatten)]
    pub args: Option<WallustArgs>,

    #[clap(subcommand)]
    pub subcmds: Option<Subcmds>,
}

/// Possible Subcommands
#[derive(Debug, clap::Subcommand)]
#[command(version, about, long_about)]
pub enum Subcmds {
    /// Apply a certain colorscheme
    Cs {
        /// Path to the file that has a colorscheme
        file: PathBuf,

        /// Don't print anything
        #[arg(short, long)]
        quiet: bool,

        /// Skip setting terminal sequences
        #[arg(short, long)]
        skip_sequences: bool,

        /// Specify a custom format. Without this option, wallust will sequentially try to decode
        /// it by trying one by one.
        #[arg(short, long)]
        format: Option<Schemes>,
    },

    /// Apply a custom built in theme
    #[cfg(feature = "themes")]
    Theme {
        /// A custom built in theme to choose from
        #[arg(value_parser = COLS_KEY, hide_possible_values(false))]
        theme: String,

        /// Don't print anything
        #[arg(short, long)]
        quiet: bool,

        /// Skip setting terminal sequences
        #[arg(short, long)]
        skip_sequences: bool,
    },
}

/// No subcommands, global arguments
#[derive(Parser, Debug, Clone)]
pub struct WallustArgs {
    /// Path to an image or json theme to use
    pub file: PathBuf,

    /// Don't print anything
    #[arg(short, long)]
    pub quiet: bool,

    /// Skip setting terminal sequences
    #[arg(short, long)]
    pub skip_sequences: bool,

    /// Generates colors even if there is a cache version of it
    //ref: <https://github.com/dylanaraps/pywal/issues/692>
    #[arg(short = 'w', long)]
    pub overwrite_cache: bool,

    /// Don't cache the results
    #[arg(short, long)]
    pub no_cache: bool,

    /// Use PATH as the config directory
    #[arg(short = 'C', long, value_name = "PATH")]
    pub config_path: Option<PathBuf>,

    /// Custom backend (ignores config file)
    #[arg(short, long, value_enum)]
    pub backend: Option<Backend>,

    /// Custom colorspace (ignores config file)
    #[arg(short, long, value_enum)]
    pub colorspace: Option<ColorSpaces>,

    /// Custom filter (ignores config file)
    #[arg(short, long, value_enum)]
    pub filter: Option<Filters>,
}
