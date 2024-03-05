//! Type declarations for working with clap `derive`, subcommands, flags, value parsers ...

use std::path::PathBuf;

use crate::{
    backends::Backend,
    colorspaces::ColorSpace,
    palettes::Palette,
    themes::Schemes,
};

use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Won't send these colors sequences
    #[arg(global = true, short, long, value_delimiter = ',', conflicts_with = "skip_sequences")]
    pub ignore_sequence: Option<Vec<Sequences>>,

    /// Don't print anything
    #[arg(global = true, short, long)]
    pub quiet: bool,

    /// Skip setting terminal sequences
    #[arg(global = true, short, long)]
    #[arg(global = true, short, long, conflicts_with = "update_current", conflicts_with = "ignore_sequence")]
    pub skip_sequences: bool,

    /// Skip templating process
    #[arg(global = true, short = 'T', long)]
    pub skip_templates: bool,

    /// Only update the current terminal
    #[arg(global = true, short, long, conflicts_with = "skip_sequences")]
    pub update_current: bool,

    /// Use FILE as the config file
    #[arg(short = 'C', long, conflicts_with = "config_dir")]
    pub config_file: Option<PathBuf>,

    /// Uses DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)
    #[arg(short = 'd', long, conflicts_with = "config_file")]
    pub config_dir: Option<PathBuf>,


    #[clap(subcommand)]
    pub subcmds: Subcmds,
}


/// Overall cli type for clap: Possible Subcommands
#[derive(Debug, Subcommand, Clone)]
#[command(version, about, long_about,
    after_help = format!("Remember to read man pages (man wallust.1, man wallust.5, ..)\nAnd the new v3 spec at {}", crate::config::V3)
    )]
pub enum Subcmds {
    /// Generate a palette from an image
    Run(WallustArgs),
    /// Apply a certain colorscheme
    Cs {
        /// Path to the file that has a colorscheme
        file: PathBuf,

        /// Specify a custom format. Without this option, wallust will sequentially try to decode
        /// it by trying one by one.
        #[arg(short, long)]
        format: Option<Schemes>,
    },

    /// Apply a custom built in theme
    #[cfg(feature = "themes")]
    Theme {
        /// A custom built in theme to choose from
        #[cfg_attr(not(feature = "buildgen"), arg(value_parser = clap::builder::ValueParser::new(col_values)))]
        #[cfg_attr(feature = "buildgen", arg(value_parser = include!(concat!(env!("OUT_DIR"), "/args.rs"))))]
        theme: String,

        /// Only preview the selected theme.
        #[arg(short, long)]
        preview: bool,
    },
    /// Migrate v2 config to v3 (might lose comments,)
    Migrate,
    /// Print information about the program and the enviroment it uses
    Debug,
}

/// No subcommands, global arguments
#[derive(Parser, Debug, Clone, Default)]
pub struct WallustArgs {
    /// Path to an image or json theme to use
    pub file: PathBuf,

    /// Alpha *template variable* value, used only for templating (default is 100)
    #[arg(short, long, value_parser = 0..=100)]
    pub alpha: Option<i64>,

    /// Choose which backend to use (overwrites config)
    #[arg(short, long, value_enum)]
    pub backend: Option<Backend>,

    /// Choose which colorspace to use (overwrites config)
    #[arg(short, long, value_enum)]
    pub colorspace: Option<ColorSpace>,

    /// Choose which fallback generation method to use (overwrites config)
    #[arg(short, long, value_enum)]
    pub fallback_generator: Option<crate::colorspaces::FallbackGenerator>,

    /// Ensure a readable contrast by checking colors in reference to the background (overwrites config)
    #[arg(short = 'k', long)]
    pub check_contrast: bool,

    /// Don't cache the results
    #[arg(short, long)]
    pub no_cache: bool,

    /// Choose which palette to use (overwrites config)
    #[arg(short, long, value_enum, value_name = "PALETTE")]
    pub palette: Option<Palette>,

    /// Add saturation from 1% to 100% (overwrites config)
    #[arg(long, value_parser = 1..=100)]
    pub saturation: Option<i64>,

    /// Choose a custom threshold, between 1 and 100 (overwrites config)
    #[arg(short, long, value_parser = 1..=100)]
    pub threshold: Option<i64>,

    /// Generates colors even if there is a cache version of it
    //ref: <https://github.com/dylanaraps/pywal/issues/692>
    #[arg(short = 'w', long)]
    pub overwrite_cache: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Sequences {
    Background,
    Foreground,
    Cursor,
    Color0,
    Color1,
    Color2,
    Color3,
    Color4,
    Color5,
    Color6,
    Color7,
    Color8,
    Color9,
    Color10,
    Color11,
    Color12,
    Color13,
    Color14,
    Color15,
}

#[cfg(all(feature = "themes", not(feature = "buildgen")))]
/// little hack to add the "random" keyword in clap
fn col_values(input: &str) -> Result<String, &'static str> {
    if input == crate::themes::RAND || wallust_themes::COLS_KEY.contains(&input) {
        Ok(input.into())
    } else {
        Err("input was not found.")
    }
}
