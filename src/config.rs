//! Config related stuff, like parsing the config file and writing templates defined on it
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

use crate::colors::Colors;
use crate::template;

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
    pub fn new(config: &Path, custom: Option<&PathBuf>) -> Result<Config> {

        // init `.config/wallust/wallust.toml`
        let config_dir = config.display().to_string() + "/wallust";
        let def_conf = PathBuf::from(config_dir.to_owned() + "/wallust.toml");

        // is the user using `--config-path`
        let (config, default_path) = match custom {
            None => (&def_conf, true),
            Some(s) => (s, false),
        };

        // Create cache dir (with all of it's parents) ONLY if the flag `--config-path` isn't in use
        if ! Path::new(&config).exists() && default_path {
            let msg = if default_path { format!("creating default one at {}", config.display()) } else { "".into() };
            eprintln!("[{}] Config file not found.. {msg}", "W".red().bold());
            fs::create_dir_all(&config_dir)?;
            File::create(config)?
                .write_all(include_str!("../wallust.toml").as_bytes())?;
        }

        toml::from_str(
            &read_to_string(config)
                .with_context(|| format!("Failed to read file {}:", config.display()))?
        ).with_context(|| format!("Failed to deserialize config file {}:", config.display()))
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

    // write entries `[[entry]]` of the config file (if any)
    pub fn write_entry(&self, config_path: &Path, img_path: &Path, colors: &Colors, quiet: bool) -> Result<()> {
        let info = "I".blue().bold().to_string();

        if let Some(s) = &self.entry {
            if ! quiet { println!("[{info}] {}: Writing templates..", "templates".magenta().bold()); }
            template::write_template(&config_path, img_path, &s, &colors, quiet)?;
        } else {
            if ! quiet { println!("[{info}] {}: No templates found", "templates".magenta().bold()); }
        }

        Ok(())
    }

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
