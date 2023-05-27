//! Config related stuff, like parsing the config file and writing templates defined on it
use std::path::Path;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

use anyhow::{Result, Context};
use owo_colors::{AnsiColors, OwoColorize};
use serde::Deserialize;

/// Representation of the toml config file `wallust.toml`
#[derive(Debug, Deserialize)]
pub struct Config {
    /// threshold to use to differentiate colors
    pub threshold: u32,
    /// Which backend to use, see backends.rs
    pub backend: crate::backends::Backend,
    /// Which filter to use, see filters.rs
    pub filter: crate::filters::Filters,
    /// Which colorspace to use, see colorspaces.rs
    pub color_space: crate::colorspaces::ColorSpaces,
    /// toml table with template and config target (optional)
    pub entry: Option<Vec<Entries>>,
}

/// An entry within the config file, toml table
/// ref: <https://toml.io/en/v1.0.0#array-of-tables>
#[derive(Debug, Deserialize)]
pub struct Entries {
    /// A file inside `~/.config/wallust/`, which is used for templating
    pub template: String,
    /// Where to write the template
    pub target: String,
}

impl Config {
    /// Constructs [`Config`] by reading the config file
    pub fn new() -> Result<Config> {
        let Some(config) = dirs::config_dir() else {
            anyhow::bail!(
                "Config path for the platform wasn't found,
please report this at <https://codeberg.org/explosion-mental/wallust/issues>");
        };
        let config_dir = config.display().to_string() + "/wallust";
        let config = config_dir.to_owned() + "/wallust.toml";

        if ! Path::new(&config).exists() {
            // Create cache dir (with all of it's parents)
            eprintln!("[{}] Config file not found.. creating default one at {config}", "W".red().bold());
            fs::create_dir_all(&config_dir)?;
            File::create(&config)?
                .write_all(include_str!("../wallust.toml").as_bytes())?;
        }

        toml::from_str(
            &read_to_string(&config)
                .with_context(|| format!("Failed to read file {}:", config))?
        ).with_context(|| format!("Failed to deserialize config file {}:", config))
    }

    pub fn print(&self) {
        println!(
"[{i}] {back_f}: Using {back} backend parser
[{i}] {th_f}: Using delta of {th} in between colors
[{i}] {cs_f}: Using {cs} colorspace variation
[{i}] {filter_f}: Using {filter} scheme filter",
            back     = self.backend.bold().color(self.backend.col()),
            th       = self.threshold.bold().color(self.threshold_col()),
            filter   = self.filter.bold().color(self.filter.col()),
            cs       = self.color_space.bold().color(self.color_space.col()),
            i        = "I".blue().bold(),
            back_f   = "image parser".magenta().bold(),
            th_f     = "threshold".magenta().bold(),
            filter_f = "scheme".magenta().bold(),
            cs_f     = "colorspace".magenta().bold(),
        );
    }
}

impl Config {
    /// thershold color for owo_colors
    pub fn threshold_col(&self) -> AnsiColors {
        match self.threshold {
            1 => AnsiColors::Yellow,
            2 => AnsiColors::Cyan,
            3..=10 => AnsiColors::Green,
            11..=49 => AnsiColors::Blue,
            50..=100 => AnsiColors::Red,
            _ => AnsiColors::Red,
        }
    }
}
