//! Config related stuff, like parsing the config file and writing templates defined on it
use serde::*;
use std::path::Path;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::fs::File;

use crate::colors::Colors;

use tinytemplate::TinyTemplate;
use anyhow::Result;
use anyhow::Context;
use owo_colors::AnsiColors;

/// Representation of the toml config file `wallust.toml`
#[derive(Debug, Deserialize)]
pub struct Config {
    /// threshold to use to differentiate colors
    pub threshold: u32,
    /// Which backend to use, see backends.rs
    pub backend: crate::backends::Backend,
    /// Which filter to use, see filters.rs
    pub filter: crate::filters::Filters,
    /// To mix colors if similar enough
    pub mix_colors: bool,
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
        let config = shellexpand::tilde("~/.config/wallust/wallust.toml");
        let config = config.as_ref();

        if ! Path::new(&config).exists() { anyhow::bail!("Config file not found, please create ~/.config/wallust/wallust.toml"); }

        toml::from_str(
            &read_to_string(config)
                .with_context(|| format!("Failed to read file {}:\n", config))?
        ).with_context(|| format!("Failed to deserialize config file {}:\n", config))
    }
}

/// Writes `template`s into `target`s
pub fn write_template(entries: &[Entries], values: &Colors) -> Result<()>{
    let config = shellexpand::tilde("~/.config/wallust/");
    let config = config.as_ref();

    let context: ColorsTemplate = values.into();

    // contents of config files
    let mut contents = vec![];

    // gather `String`s of the contents of the entries (in order to cast it down to &str)
    for e in entries {
        let path = config.to_owned() + &e.template;
        //println!("->'{}'", &path);
        contents.push(
            (&e.target, read_to_string(&path)
                        .with_context(|| format!("Failed to read file {}:\n", path))?
             )
        );
    }

    let mut tt = TinyTemplate::new();

    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path
    for (path, stuff) in &contents {
        tt.add_template("colors", stuff)?;
        let rendered = tt.render("colors", &context)?;
        //XXX on `shellexpand`, think about using `::full()` to support env vars. Seems a bit sketchy/sus
        let mut buffer = File::create(shellexpand::tilde(path).as_ref())
            .with_context(|| format!("Failed to create file {}:\n", path))?;
        buffer.write_all(rendered.as_bytes())
            .with_context(|| format!("Failed to write to file {}:\n", path))?;
        //println!("FROM: '{path}' --- '{}'", rendered);
    }
    Ok(())
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

/// Simply a copy of [`Colors`]
/// (to avoid working with generics, since there is no need to complicate this)
#[derive(Serialize, Deserialize)]
struct ColorsTemplate {
    background: String,
    foreground: String,
    color0 : String,
    color1 : String,
    color2 : String,
    color3 : String,
    color4 : String,
    color5 : String,
    color6 : String,
    color7 : String,
    color8 : String,
    color9 : String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
}

/// From implementation trait for the [`Colors`] with [`Myrgb`] type to a String for TinyTemplate
/// to use
impl From<&Colors> for ColorsTemplate {
    fn from(c: &Colors) -> Self {
        Self {
            background : c.background.to_string(),
            foreground : c.foreground.to_string(),
            color0  : c.color0.to_string(),
            color1  : c.color1.to_string(),
            color2  : c.color2.to_string(),
            color3  : c.color3.to_string(),
            color4  : c.color4.to_string(),
            color5  : c.color5.to_string(),
            color6  : c.color6.to_string(),
            color7  : c.color7.to_string(),
            color8  : c.color8.to_string(),
            color9  : c.color9.to_string(),
            color10 : c.color10.to_string(),
            color11 : c.color11.to_string(),
            color12 : c.color12.to_string(),
            color13 : c.color13.to_string(),
            color14 : c.color14.to_string(),
            color15 : c.color15.to_string(),
        }
    }
}
