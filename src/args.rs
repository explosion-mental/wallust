//! Type declarations for working with clap `derive`, subcommands, flags, value parsers ...

use std::path::PathBuf;

use crate::{
    backends::Backend,
    colorspaces::ColorSpace,
    palettes::Palette,
    themes::Schemes,
};

use clap::Parser;
use serde::Deserialize;


/// Overall cli type for clap: Possible Subcommands
#[derive(Debug, Parser)]
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

        /// Won't send these colors sequences
        #[arg(short, long, value_delimiter = ',', conflicts_with = "skip_sequences")]
        ignore_sequence: Option<Vec<Sequences>>,

        /// Don't print anything
        #[arg(short, long)]
        quiet: bool,

        /// Skip setting terminal sequences
        #[arg(short, long)]
        skip_sequences: bool,

        /// Skip templating process
        #[arg(short = 'T', long, conflicts_with = "update_current", conflicts_with = "ignore_sequence")]
        skip_templates: bool,

        /// Only update the current terminal
        #[arg(short, long, conflicts_with = "skip_sequences")]
        update_current: bool,
    },

    /// Apply a custom built in theme
    #[cfg(feature = "themes")]
    Theme {
        /// A custom built in theme to choose from
        #[cfg_attr(not(feature = "buildgen"), arg(value_parser = clap::builder::ValueParser::new(col_values)))]
        #[cfg_attr(feature = "buildgen", arg(value_parser = include!(concat!(env!("OUT_DIR"), "/args.rs"))))]
        theme: String,

        /// Won't send these colors sequences
        #[arg(short, long, value_delimiter = ',', conflicts_with = "skip_sequences")]
        ignore_sequence: Option<Vec<Sequences>>,

        /// Only preview the selected theme.
        #[arg(short, long, conflicts_with = "quiet")]
        preview: bool,

        /// Don't print anything
        #[arg(short, long)]
        quiet: bool,

        /// Skip setting terminal sequences
        #[arg(short, long)]
        skip_sequences: bool,

        /// Skip templating process
        #[arg(short = 'T', long, conflicts_with = "update_current", conflicts_with = "ignore_sequence")]
        skip_templates: bool,

        /// Only update the current terminal
        #[arg(short, long, conflicts_with = "skip_sequences")]
        update_current: bool,
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

    /// Use FILE as the config file
    #[arg(short = 'C', long, value_name = "CONFIG_FILE")]
    pub config_path: Option<PathBuf>,

    /// Use DIR as the config directory
    #[arg(short = 'd', long, conflicts_with = "config_path")]
    pub config_dir: Option<PathBuf>,

    /// Choose which generation method to use (overwrites config)
    #[arg(short, long, value_enum)]
    pub generation: Option<crate::colorspaces::Generate>,

    /// Won't send these colors sequences
    #[arg(short, long, value_delimiter = ',', conflicts_with = "skip_sequences")]
    pub ignore_sequence: Option<Vec<Sequences>>,

    /// Ensure a readable contrast by checking colors in reference to the background (overwrites config)
    #[arg(short = 'k', long)]
    pub check_contrast: bool,

    /// Don't cache the results
    #[arg(short, long)]
    pub no_cache: bool,

    /// Choose which palette to use (overwrites config)
    #[arg(short, long, value_enum, value_name = "PALETTE")]
    pub palette: Option<Palette>,

    /// Don't print anything
    #[arg(short, long)]
    pub quiet: bool,

    /// Skip setting terminal sequences
    #[arg(short, long, conflicts_with = "update_current", conflicts_with = "ignore_sequence")]
    pub skip_sequences: bool,

    /// Add saturation from 1% to 100% (overwrites config)
    #[arg(long, value_parser = 1..=100)]
    pub saturation: Option<i64>,

    /// Choose a custom threshold, between 1 and 100 (overwrites config)
    #[arg(short, long, value_parser = 1..=100)]
    pub threshold: Option<i64>,

    /// Skip the templating process
    #[arg(short = 'T', long)]
    pub skip_templates: bool,

    /// Only update the current terminal colros
    #[arg(short, long, conflicts_with = "skip_sequences")]
    pub update_current: bool,

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
