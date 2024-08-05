//! Config related stuff, like parsing the config file and writing templates defined on it
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::fs::read_to_string;

use crate::args::WallustArgs;
use crate::args::Globals;
use crate::colors::Colors;
use crate::template;
use crate::template::TemplateFields;

use anyhow::{Result, Context};
use owo_colors::{AnsiColors, OwoColorize};
use serde::Deserialize;

/// Representation of the toml config file `wallust.toml`
///
/// Maybe divide into `Internal` and `TomlConfig`.
#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "doc" , derive(documented::Documented, documented::DocumentedFields))]
pub struct Config {
    /// threshold to use to differentiate colors
    #[serde(default)]
    #[serde(deserialize_with = "validate_threshold")]
    pub threshold: Option<u8>,
    /// Which backend to use, see backends.rs
    #[serde(rename = "backend")]
    pub backend_user: Option<crate::backends::Backend>,
    /// Which palette to use, see palettes.rs
    #[serde(rename = "palette")]
    pub palette_user: Option<crate::palettes::Palette>,
    /// Which colorspace to use, see colorspaces.rs
    #[serde(rename = "color_space")]
    pub color_space_user: Option<crate::colorspaces::ColorSpace>,
    /// Optional alpha value
    pub alpha: Option<u8>,
    /// This flags ensures good contrast between images, by doing some w3m calculations.
    /// However it isn't required and should only be turn on when you notice bad contrast between many images.
    pub check_contrast: Option<bool>,
    /// Maybe the user requires more vivid colors
    pub saturation: Option<u8>,
    /// How to 'generate' colors when there aren't enough colors to create the `palette`.
    /// This appears as "Artificially generating colors.." in cli
    pub fallback_generator: Option<crate::colorspaces::FallbackGenerator>,
    /// templates: a new way of defining templates, giving the ability of naming stuff.
    // [templates]
    // dunst.src = 'C:\long\path'
    // dunst.dst = '~/.config/dunst'
    // zathura = { src = 'zathura.rc', dst = '~/.config/zathura' }
    pub templates: Option<HashMap<String, Fields>>,
    /// Allows to change the path of the templates
    ///  default: "~/.config/wallust/templates/"
    //pub templates_dir: PathBuf,

    #[deprecated]
    /// TOML: array of tables for "template" and "target"
    /// This is here only for `wallust migrate`
    pub entry: Option<Vec<Entries>>,

    /// Config directory (wallust/) path
    #[serde(skip)]
    pub dir: PathBuf,

    /// Config file (wallust.toml) path
    #[serde(skip)]
    pub file: PathBuf,

    /// template directory (wallust/template/) path
    #[serde(skip)]
    pub templates_dir: PathBuf,

    /* TRUE VALUES , used internally */

    /// True threshold gathered from threshold.
    #[serde(skip)]
    pub true_th: u8,

    #[serde(skip)]
    pub backend: crate::backends::Backend,

    #[serde(skip)]
    pub color_space: crate::colorspaces::ColorSpace,

    #[serde(skip)]
    pub palette: crate::palettes::Palette,
}

/// An entry within the config file, toml table
/// ref: <https://toml.io/en/v1.0.0#array-of-tables>
#[derive(Debug, Deserialize, Clone)]
pub struct Fields {
    /// A file inside `~/.config/wallust/`, which is used for templating
    #[serde(alias = "src")]
    pub template: String,
    /// Where to write the template
    #[serde(alias = "dst")]
    pub target: String,
    /// Allows pywal template spec compatibility (disabled by default)
    pub pywal: Option<bool>,
    // If 'src' is a directory, 'dst' SHOULD also be one.
    // This flag allows for 'src', when a dir, to be templated recursively
    // If 'src' is a file, this has no effect.
    //TODO implement recursive behaviour
    //pub recursive: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Entries {
    /// A file inside `~/.config/wallust/`, which is used for templating
    pub template: String,
    /// Where to write the template
    pub target: String,
    /// Allows pywal template spec compatibility (disabled by default)
    pub new_engine: Option<bool>,
}

/// How to populate `wallpaper` template value:
/// 1. With `wallust theme rose-pine`, it will use the name of the theme in use. (e.g. `rose-pine`)
/// 2. With `wallust cs scheme.json`, it will use the absolute path of the file used. (e.g. `/home/user/scheme.json`)
/// 3. Normal behaviour with `wallust run image.png`, it will use the wallpaper absolute path. (e.g. `/home/user/image.png`)
pub enum WalStr<'a> {
    Path(&'a Path),
    Theme(&'a str),
}

/// v3.md link
pub const V3: &str = "<https://explosion-mental.codeberg.page/wallust/v3.html>";

impl Config {
    /// Constructs [`Config`] by reading the config file
    pub fn new(g: &Globals) -> Result<Config> {

        // first check if user give out a custom config-dir
        let dir = match &g.config_dir {
            Some(s) => s,
            None => {
                let Some(original_config_path) = dirs::config_dir() else {
                    anyhow::bail!("Config path for the platform could not be found.");
                };
                &original_config_path.join("wallust")
            }
        };

        // then we check for a custom config-file
        let config = match &g.config_file {
            Some(s) => {
                // check if exist first, since we don't need a config file,
                // we use default Configuration options when it doesn't
                // but since this is a CLI FLAG, make sure it exist first.
                if !s.exists() { anyhow::bail!("Configuration file provided doesn't exist: {}", s.display()); }
                s
            },
            // if not, we use the default path: `dir/wallust.toml`
            None => &dir.join("wallust.toml"),
        };

        let templates_dir = match &g.templates_dir {
            Some(s) => {
                // check if exist first
                if !s.exists() { anyhow::bail!("Templates dir provided doesn't exist: {}", s.display()); }
                s
            },
            None => &dir.join("templates"),
        };

        let mut ret = if !config.exists() { // don't care if it doesn't exist.
            println!("[{info}] {t}: No configuration file found, using default values.", info = "I".blue().bold(), t = "config".magenta().bold());
            Config::default()
        } else {
            let s = || format!("Failed to read file {}:\nIf you are switching from v2 to v3, use `wallust migrate`.\nMake sure to read {V3} as well.", config.display());
            toml::from_str(
                &read_to_string(config).with_context(s)?
            ).with_context(s)?
        };

        ret.templates_dir = templates_dir.into();
        ret.dir = dir.into();
        ret.file = config.into();
        ret.true_th = 0; //dummy placeholder

        // defined or defaults.
        ret.backend = ret.backend_user.unwrap_or_default();
        ret.color_space = ret.color_space_user.unwrap_or_default();
        ret.palette = ret.palette_user.unwrap_or_default();

        //println!("{:#?}", ret);

        Ok(ret)
    }

    pub fn print(&self) {
        let empty = String::new();
        let k = if self.check_contrast.unwrap_or(false) {
            format!("\n[{}] {}: Doing extra calculations to ensure a good contrast",
                "I".blue().bold(),
                "contrast".magenta().bold()
                )
        } else { empty.clone() };

        let sat = if let Some(s) = self.saturation {
            format!("\n[{}] {}: Adding saturation to existing palette by {s}%",
                "I".blue().bold(),
                "saturation".magenta().bold()
                )
        } else { empty };

        let th = match self.threshold {
            Some(s) => format!("Using a threshold of {s} in between colors."),
            None => format!("Not defined, using {} default thresholds.", "best".bold()),
        };

        println!(
"[{i}] {back_f}: Using {back} backend parser
[{i}] {th_f}: {th}
[{i}] {cs_f}: Using {cs} colorspace variation
[{i}] {palette_f}: Using {palette} palette{k}{sat}",
            back     = self.backend.bold().color(self.backend.col()),
            palette  = self.palette.bold().color(self.palette.col()),
            cs       = self.color_space.bold().color(self.color_space.col()),
            i        = "I".blue().bold(),
            back_f   = "image parser".magenta().bold(),
            th_f     = "threshold".magenta().bold(),
            palette_f = "scheme palette".magenta().bold(),
            cs_f     = "colorspace".magenta().bold(),
        );
    }

    /// Writes templates defined in the config file (if any)
    /// Should print a warning if you are using the old `[[entry]]` syntax (since it's going to be deprecated in v3).
    pub fn write_entry(&self, wal_str: &WalStr, colors: &Colors, quiet: bool) -> Result<()> {
        let init = format!("[{info}] {t}: ", info = "I".blue().bold(), t = "templates".magenta().bold());

        let templates_header = match &self.templates {
            Some(s) => {
                if ! quiet { println!("{init}Writing templates.."); }
                s
            },
            None => {
                if ! quiet { println!("{init}No templates found"); }
                return Ok(())
            },
        };

        // check if themes exist, if it does we are using the `theme` subcommand,
        // which means there is not image path, so use the theme name as for the `wallpaper` value
        let image_path = match wal_str {
            // use the theme name otherwise
            WalStr::Theme(s) => s.to_string(),
            // make sure to display the absolute path of the wallpaper
            WalStr::Path(p) => dunce::canonicalize(p).expect("PATH EXIST, validation from clap").display().to_string(),
        };

        let values = TemplateFields {
            alpha: self.alpha.unwrap_or(100),
            backend: &self.backend,
            colorspace: &self.color_space,
            palette: &self.palette,
            image_path: &image_path,
            colors,
        };

        template::write_template(&self.templates_dir, templates_header, &values, quiet)
    }

    /// if the user provides this values in the cli, overwrite the [`Config`] configuration
    pub fn customs_cli(&mut self, cli: &WallustArgs) {
        if let Some(b) = cli.backend {
            self.backend = b;
        }

        if let Some(col) = cli.colorspace {
            self.color_space = col;
        }

        if let Some(f) = cli.palette {
            self.palette = f;
        }

        if let Some(t) = cli.threshold {
            self.threshold = Some(t as u8); //t is [1..=100]
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

        if let Some(g) = cli.fallback_generator {
            self.fallback_generator = Some(g);
        }
    }

    /// thershold color for owo_colors
    pub fn threshold_col(&self) -> AnsiColors {
        match self.true_th {
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

            let temps = if let Some(e) = &self.templates {
                let mut s = String::new();
                for i in e {
                    let pywal = if let Some(s) = i.1.pywal {
                        format!("{sp}{sp}pywal = {s}\n")
                    } else {
                        String::new()
                    };

                    let name = i.0;

                    s.push_str(
                        &format!("{sp}{name}\n{sp}{sp}template = {}\n{sp}{sp}target   = {}\n{pywal}",
                                i.1.template, i.1.target)
                        );
                }
                s.trim_end().to_owned()
            } else {
                String::new()
            };

            let templates = if temps.is_empty() {
                "No entries found.".into()
            } else {
                temps
            };

            write!(f, "\
Config directory: {dir}
Config file: {file}
Configuration options:
    backend        = {b}
    color_space    = {c}
    threshold      = {t:?}
    palette        = {f}
    check_contrast = {con:?}
    saturation     = {sat:?}
    alpha          = {a:?}
Templates:
{templates}",
            b = self.backend,
            c = self.color_space,
            t = self.threshold,
            f = self.palette,
            con = self.check_contrast,
            sat = self.saturation,
            a = self.alpha,
            dir = self.dir.display(),
            file = self.file.display(),
            )
    }
}

fn validate_threshold<'de, D>(d: D) -> Result<Option<u8>, D::Error>
    where D: serde::de::Deserializer<'de>
{
    use serde::de;


    let value = Option::deserialize(d)?;
    let value = match value {
        Some(s) => s,
        None => return Ok(None),
    };

    if value <= 100 { return Ok(Some(value)); }

    Err(de::Error::invalid_value(de::Unexpected::Unsigned(value as u64), &"a value between 0 and 100."))
}
