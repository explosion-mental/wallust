//! Cli flags
//! * consider using the same flags as `pywal`, in order to be a drop-in replacement..

use std::path::PathBuf;

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
    /// Apply a certain theme/colorscheme
    Cs {
        #[arg(value_parser = crate::themes::COLS_KEY, hide_possible_values(false))]
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
    #[arg(short = 'c', long)]
    pub overwrite_cache: bool,

    /// Don't cache the results
    #[arg(short, long)]
    pub no_cache: bool,
}
