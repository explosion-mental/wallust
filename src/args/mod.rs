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

pub mod migrate;

pub use migrate::migrate;

/// These flags can go before AND after the subcommand, like `wallust -q run image.png` or `wallust run image.png -q`
#[derive(Debug, Parser, Default)]
pub struct Globals {
    /// Won't send these colors sequences
    #[arg(global = true, short = 'I', long, value_delimiter = ',', conflicts_with = "skip_sequences")]
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

    /// Use CONFIG_FILE as the config file
    #[arg(global = true, short = 'C', long, conflicts_with = "config_dir")]
    pub config_file: Option<PathBuf>,

    /// Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)
    #[arg(global = true, short = 'd', long, conflicts_with = "config_file", conflicts_with = "templates_dir")]
    pub config_dir: Option<PathBuf>,

    /// Uses TEMPLATE_DIR as the template directory.
    #[arg(global = true, long, conflicts_with = "config_dir")]
    pub templates_dir: Option<PathBuf>,

    /// Won't read the config and avoids creating it's config path.
    #[arg(global = true, short = 'N', long, conflicts_with = "config_file", conflicts_with = "config_dir")]
    pub no_config: bool,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(flatten)]
    pub globals: Globals,

    #[clap(subcommand)]
    pub subcmds: Subcmds,
}


/// Overall cli type for clap: Possible Subcommands
#[derive(Debug, Subcommand, Clone)]
#[command(about, long_about,
    version = include!(concat!(env!("OUT_DIR"), "/version.rs")),
    after_help = format!("Remember to read man pages (man wallust.1, man wallust.5, ..)\nAnd the new v3 spec at {}", crate::config::V3),
    )]
pub enum Subcmds {
    /// Generate a palette from an image
    Run(WallustArgs),
    /// Apply a certain colorscheme
    Cs {
        /// Name of the scheme inside `wallust/colorschemes` directory or the name of a theme.
        colorscheme: String,

        /// Specify a custom format. Without this option, wallust will sequentially try to decode
        /// it by trying one by one.
        #[arg(short, long)]
        format: Option<Schemes>,
    },

    /// Apply a custom built in theme
    #[cfg(feature = "themes")]
    Theme {
        /// A custom built in theme to choose from
        // #[cfg_attr(feature = "buildgen", arg(value_parser = clap::builder::ValueParser::new(col_values)))]
        // #[cfg_attr(not(feature = "buildgen"), arg(value_parser = include!(concat!(env!("OUT_DIR"), "/args.rs"))))]
        #[arg(value_parser = include!(concat!(env!("OUT_DIR"), "/args.rs")))]
        theme: String,

        /// Only preview the selected theme.
        #[arg(short, long)]
        preview: bool,
    },
    /// Migrate v2 config to v3 (might lose comments,)
    Migrate,
    /// Print information about the program and the enviroment it uses
    Debug,

    /// A drop-in cli replacement for pywal
    Pywal(PywalArgs),

}

/// No subcommands, global arguments
#[derive(Parser, Debug, Clone, Default)]
pub struct WallustArgs {
    /// Path to the image to use
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
    #[arg(short, long, value_enum)]
    pub palette: Option<Palette>,

    /// Add saturation from 1% to 100% (overwrites config)
    #[arg(long, value_parser = 1..=100)]
    pub saturation: Option<i64>,

    /// Choose a custom threshold, between 1 and 100 (overwrites config)
    #[arg(short, long, value_parser = 1..=100)]
    pub threshold: Option<i64>,

    /// Dynamically changes the threshold to be best fit
    #[arg(long, conflicts_with = "threshold")]
    pub dynamic_threshold: bool,

    /// Generates colors even if there is a cache version of it
    //ref: <https://github.com/dylanaraps/pywal/issues/692>
    #[arg(short = 'w', long)]
    pub overwrite_cache: bool,
}

/// Pywal cli flags arguments. This is to create a drop in replacement, since many apps rely on the
/// `pywal` command. However, cli flags are ignored, as of now.
#[derive(Parser, Debug, Clone, Default)]
#[command(about, long_about)]
pub struct PywalArgs {
    /// Set terminal background transparency. *Only works in URxvt*
    #[arg(short, value_name = "alpha")]
    pub alpha: Option<String>,

    /// Custom background color to use.
    #[arg(short, value_name = "background")]
    pub background: Option<String>,

    /// Which color backend to use
    #[arg(long, value_name = "[backend]")]
    pub backend: Option<String>,

    //  --theme [/path/to/file or theme_name], -f [/path/to/file or theme_name]
    /// Which colorscheme file to use. Use 'wal --theme' to list builtin themes.
    #[arg(short = 'f', long, value_name = "theme")]
    pub theme: Option<String>,

    /// When pywal is given a directory as input and this flag is used: Go through the images in order instead of shuffled.
    #[arg(long)]
    pub iterative: bool,

    /// Set the color saturation.
    #[arg(long, value_name = "0.0 - 1.0")]
    pub saturate: Option<f32>,

    /// Print the current color palette.
    #[arg(long)]
    pub preview: bool,

    /// Fix text-artifacts printed in VTE terminals.
    #[arg(long)]
    vte: bool,

    /// Delete all cached colorschemes.
    #[arg(short = 'c')]
    pub clean_cache: bool,

    /// Which image or directory to use.
    #[arg(required_unless_present = "theme")]
    #[arg(short = 'i', value_name = "/path/to/img.jpg")]
    pub file: Option<PathBuf>,

    /// Generate a light colorscheme.
    #[arg(short = 'l')]
    pub light: bool,

    /// Skip setting the wallpaper.
    #[arg(short = 'n')]
    pub no_wallpaper: bool,

    /// External script to run after "wal".
    #[arg(short = 'o', value_name = "script_name")]
    pub othercmd: Option<String>,

    /// Quiet mode, don't print anything.
    #[arg(short = 'q')]
    pub quiet: bool,

    /// 'wal -r' is deprecated: Use (cat ~/.cache/wal/sequences &) instead.
    #[arg(short = 'r')]
    pub r: bool,

    /// Restore previous colorscheme.
    #[arg(short = 'R')]
    pub restore: bool,

    /// Skip changing colors in terminals.
    #[arg(short = 's')]
    pub skip_sequences: bool,

    /// Skip changing colors in tty.
    #[arg(short = 't')]
    pub tty: bool,

    /// Print "wal" version.
    #[arg(short = 'v')]
    pub version: bool,

    /// Skip reloading gtk/xrdb/i3/sway/polybar
    #[arg(short = 'e')]
    pub e: bool,
}

/// Convert PywalArgs to WallustArgs
impl From<PywalArgs> for WallustArgs {
    fn from(p: PywalArgs) -> Self {
        Self {
            // All empty so wallust prioritizes the config file
            alpha: None,
            backend: None,
            colorspace: None,
            check_contrast: false,
            dynamic_threshold: true,
            fallback_generator: None,
            no_cache: false,
            overwrite_cache: false,
            palette: None,
            saturation: None,
            threshold: None,
            file: p.file.expect("ALWAYS SOME, CHECKED ON MAIN"),
        }
    }
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
