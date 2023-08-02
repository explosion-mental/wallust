//! Config related stuff, like parsing the config file and writing templates defined on it
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

use crate::args::WallustArgs;
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

    /// Config directory (wallust/) path
    #[serde(skip)]
    pub path: PathBuf,

    /// Config file (wallust.toml) path
    #[serde(skip)]
    pub file: PathBuf,

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
    pub fn new(original_config_path: &PathBuf, args: Option<&WallustArgs>) -> Result<Config> {

        // check config file or generate one if not one isn't found
        let config_cli = match args {
            Some(s) => s.config_path.as_ref(),
            None => None,
        };

        let mut is_orig_conf = true;

        // check config dir
        let config_path = match args {
            Some(s) => {
                match &s.config_dir {
                    Some(path) => { //only in this case, the config dir is altered
                        is_orig_conf = false;
                        path
                    },
                    None => &original_config_path,
                }
            },
            None => &original_config_path,
        };

        let is_original = is_orig_conf;
        let config = config_path;
        let custom = config_cli;

        // init `.config/wallust/wallust.toml`
        let join_dir = if is_original { "wallust" } else { "" };

        let config_dir = config.join(join_dir);
        let def_conf = config_dir.join("wallust.toml");

        // is the user using `--config-path`
        let (config, default_path) = match custom {
            None => (&def_conf, true),
            Some(s) => (s, false),
        };

        // Create cache dir (with all of it's parents) ONLY if the flag `--config-path` isn't in use
        if ! Path::new(&config).exists() && default_path && is_original {
            let msg = if default_path { format!("creating default one at {}", config.display()) } else { "".into() };
            eprintln!("[{}] Config file not found.. {msg}", "W".red().bold());
            fs::create_dir_all(&config_dir)?;
            File::create(config)?
                .write_all(include_str!("../wallust.toml").as_bytes())?;
        }

        let mut ret: Config = toml::from_str(
            &read_to_string(config)
                .with_context(|| format!("Failed to read file {}:", config.display()))?
        ).with_context(|| format!("Failed to deserialize config file {}:", config.display()))?;

        ret.path = config_dir;
        ret.file = config.to_path_buf();

        Ok(ret)
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
    pub fn write_entry(&self, img_path: &Path, colors: &Colors, quiet: bool) -> Result<()> {
        let info = "I".blue().bold().to_string();

        if let Some(s) = &self.entry {
            if ! quiet { println!("[{info}] {}: Writing templates..", "templates".magenta().bold()); }
            template::write_template(self, img_path, s, colors, quiet)?;
        } else {
            if ! quiet { println!("[{info}] {}: No templates found", "templates".magenta().bold()); }
        }

        Ok(())
    }

    /// if the user provides this values in the cli, overwrite the [`Config`] configuration
    pub fn customs_cli(&mut self, cli: &WallustArgs) {
        if let Some(b) = cli.backend {
            self.backend = b;
        }

        if let Some(col) = cli.colorspace {
            self.color_space = col;
        }

        if let Some(f) = cli.filter {
            self.filter = f;
        }
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
