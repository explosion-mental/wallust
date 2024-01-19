//! Config related stuff, like parsing the config file and writing templates defined on it
use std::collections::HashMap;
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
//TODO wallust should be able to work without a config file?
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
    /// Config directory (wallust/) path
    #[serde(skip)]
    pub dir: PathBuf,

    /// Config file (wallust.toml) path
    #[serde(skip)]
    pub file: PathBuf,

    // templates: a new way of defining templates, giving the ability of naming stuff.
    // [templates]
    // dunst.src = 'C:\long\path'
    // dunst.dst = '~/.config/dunst'
    // zathura = { src = 'zathura.rc', dst = '~/.config/zathura' }
    pub templates: Option<HashMap<String, Entries>>,
}

/// An entry within the config file, toml table
/// ref: <https://toml.io/en/v1.0.0#array-of-tables>
#[derive(Debug, Deserialize, Clone)]
pub struct Entries {
    /// A file inside `~/.config/wallust/`, which is used for templating
    #[serde(alias = "src")]
    pub template: String,
    /// Where to write the template
    #[serde(alias = "dst")]
    pub target: String,
    /// Whether to use the new method or not
    pub new_engine: Option<bool>,
}

/// How to populate `wallpaper` template value:
/// 1. With `wallust theme rose-pine`, it will use the name of the theme in use. (e.g. `rose-pine`)
/// 2. With `wallust cs scheme.json`, it will use the absolute path of the file used. (e.g. `/home/user/scheme.json`)
/// 3. Normal behaviour with `wallust image.png`, it will use the wallpaper absolute path. (e.g. `/home/user/image.png`)
pub enum WalStr<'a> {
    Path(&'a Path),
    Theme(&'a str),
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
                .write_all(include_bytes!("../wallust.toml"))?;
        }

        let mut ret: Config = toml::from_str(
            &read_to_string(config)
                .with_context(|| format!("Failed to read file {}:", config.display()))?
        ).with_context(|| format!("Failed to deserialize config file {}:", config.display()))?;

        ret.dir = config_dir;
        ret.file = config.to_path_buf();

        //println!("{:#?}", ret);

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
    pub fn write_entry(&self, wal_str: &WalStr, colors: &Colors, quiet: bool) -> Result<()> {
        let info = "I".blue().bold().to_string();

        // check if themes exist, if it does we are using the `theme` subcommand,
        // which means there is not image path, so use the theme name as for the `wallpaper` value
        let wallpaper_str = match wal_str {
            // use the theme name otherwise
            WalStr::Theme(s) => s.to_string(),
            // make sure to display the absolute path of the wallpaper
            WalStr::Path(p) => std::fs::canonicalize(p).expect("PATH EXIST, validation from clap").display().to_string(),
        };

        let mut entries = vec![];

        if let Some(s) = &self.entry {
            entries.extend(s.clone());
        }

        //TODO on next major version, use the name assign to the template. think it needs to be included in `write_template()`
        if let Some(s) = &self.templates {
            let e = s.clone().into_values().collect::<Vec<Entries>>();
            entries.extend(e);
        }

        if !entries.is_empty() {
            if ! quiet { println!("[{info}] {}: Writing templates..", "templates".magenta().bold()); }
            template::write_template(self, &wallpaper_str, &entries, colors, quiet)
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

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let sp = "    ";
            let entry = if let Some(e) = &self.entry {
                let mut s = String::new();
                for i in e {
                    let new_engine = if let Some(s) = i.new_engine {
                        format!("{sp}{sp}new_engine = {s}\n")
                    } else {
                        "".into()
                    };

                    s.push_str(
                        &format!("{sp}[[entry]]\n{sp}{sp}template = {}\n{sp}{sp}target   = {}\n{new_engine}",
                                i.template, i.target)
                        );
                }
                s.trim_end().to_owned()
            } else {
                "No entries found.".into()
            };

            write!(f, "\
Config directory: {dir}
Config file: {file}
Configuration options:
    backend        = {b}
    color_space    = {c}
    threshold      = {t}
    filter         = {f}
    check_contrast = {con:?}
    saturation     = {sat:?}
    alpha          = {a:?}
Templates:
{entry}",
            b = self.backend,
            c = self.color_space,
            t = self.threshold,
            f = self.filter,
            con = self.check_contrast,
            sat = self.saturation,
            a = self.alpha,
            dir = self.dir.display(),
            file = self.file.display(),
            )
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
