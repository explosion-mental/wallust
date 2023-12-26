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
#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "makeconfig", derive(documented::Documented, documented::DocumentedFields))]
pub struct Config {
    /// threshold to use to differentiate colors
    #[serde(deserialize_with = "validate_threshold")]
    pub threshold: u8,
    /// Which backend to use, see backends.rs
    pub backend: crate::backends::Backend,
    /// Which filter to use, see filters.rs
    pub filter: crate::filters::Filters,
    /// Which colorspace to use, see colorspaces.rs
    pub color_space: crate::colorspaces::ColorSpaces,
    /// toml table with template and config target (optional)
    pub entry: Option<Vec<Entries>>,
    /// Optional alpha value
    pub alpha: Option<u8>,
    /// This flags ensures good contrast between images, by doing some w3m calculations.
    /// However it isn't required and should only be turn on when you notice bad contrast between many images.
    pub check_contrast: Option<bool>,
    /// Maybe the user requires more vivid colors
    pub saturation: Option<u8>,
    /// Whether to use the new method or not
    pub new_engine: Option<bool>,

    /// Config directory (wallust/) path
    #[serde(skip)]
    pub dir: PathBuf,

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
        let custom = match args {
            Some(s) => s.config_path.as_ref(),
            None => None,
        };

        // true -> uses original_config_path
        // false -> uses a custom path
        let mut is_original = true;

        // check config dir
        let config = match args {
            Some(s) => {
                match &s.config_dir {
                    Some(path) => { //only in this case, the config dir is altered
                        is_original = false;
                        path
                    },
                    None => original_config_path,
                }
            },
            None => original_config_path,
        };

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

        ret.dir = config_dir;
        ret.file = config.to_path_buf();

        Ok(ret)
    }

    pub fn print(&self) {

        let k = if self.check_contrast.unwrap_or(false) {
            format!("\n[{}] {}: Doing extra calculations to ensure a good contrast",
                "I".blue().bold(),
                "contrast".magenta().bold()
                )
        } else { "".to_string() };

        let sat = if let Some(s) = self.saturation {
            format!("\n[{}] {}: Adding saturation to existing palette by {s}%",
                "I".blue().bold(),
                "saturation".magenta().bold()
                )
        } else {
            "".to_string()
        };

        println!(
"[{i}] {back_f}: Using {back} backend parser
[{i}] {th_f}: Using delta of {th} in between colors
[{i}] {cs_f}: Using {cs} colorspace variation
[{i}] {filter_f}: Using {filter} scheme filter{k}{sat}",
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
            template::write_template(self, img_path, s, colors, quiet)
        } else {
            if ! quiet { println!("[{info}] {}: No templates found", "templates".magenta().bold()); }
            Ok(())
        }
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

        if let Some(t) = cli.threshold {
            self.threshold = t as u8; //t is [1..=100]
        }

        if let Some(a) = cli.alpha {
            self.alpha = Some(a as u8);
        }

        if cli.check_contrast {
            self.check_contrast = Some(cli.check_contrast);
        }

        if let Some(sat) = cli.saturation {
            self.saturation = Some(sat as u8);
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

fn validate_threshold<'de, D>(d: D) -> Result<u8, D::Error>
    where D: serde::de::Deserializer<'de>
{
    use serde::de;

    let value = u8::deserialize(d)?;

    if value <= 100 { return Ok(value); }

    Err(de::Error::invalid_value(de::Unexpected::Unsigned(value as u64), &"a value between 0 and 100."))
}
