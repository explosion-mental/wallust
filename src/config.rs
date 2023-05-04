//! Config related stuff, like parsing the config file and writing templates defined on it
use std::path::Path;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use crate::colors::Colors;

use anyhow::{Result, Context};
use owo_colors::AnsiColors;
use new_string_template::template::Template;
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
        let config = config.display().to_string() + "/wallust/wallust.toml";


        // XXX maybe generate a sample config file if one isn't found
        if ! Path::new(&config).exists() {
            anyhow::bail!(
                "Config file not found, please create ~/.config/wallust/wallust.toml
Check out <https://codeberg.org/explosion-mental/wallust/src/branch/master/wallust.toml> for a documented example."
            );
        }

        toml::from_str(
            &read_to_string(&config)
                .with_context(|| format!("Failed to read file {}:", config))?
        ).with_context(|| format!("Failed to deserialize config file {}:", config))
    }
}

/// Writes `template`s into `target`s
pub fn write_template(entries: &[Entries], values: &Colors, quiet: bool) -> Result<()>{
    let Some(config) = dirs::config_dir() else {
        anyhow::bail!(
            "Config path for the platform wasn't found,
please report this at <https://codeberg.org/explosion-mental/wallust/issues>");
    };
    let config = config.display().to_string() + "/wallust/";

    // contents of config files
    let mut contents = vec![];

    // gather `String`s of the contents of the entries (in order to cast it down to &str)
    for e in entries {
        let path = config.to_owned() + &e.template;
        contents.push(
            (&e.target, read_to_string(&path)
                        .with_context(|| format!("Failed to read file {}:", path))?
             )
        );
    }

    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path
    for (target, file_content) in &contents {
        let rendered = Template::new(file_content).render(&values.to_hash())
            .with_context(|| format!("Templating failed with {}:", target))?;

        //XXX on `shellexpand`, think about using `::full()` to support env vars. Seems a bit sketchy/sus
        let mut buffer = File::create(shellexpand::tilde(target).as_ref())
            .with_context(|| format!("Failed to create file {}:", target))?;

        buffer.write_all(rendered.as_bytes())
            .with_context(|| format!("Failed to write to file {}:", target))?;
        if ! quiet { println!("    * {} ... OK", target); }
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

impl Colors {
    pub fn to_hash(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        map.insert("color0" , self.color0 .to_string());
        map.insert("color1" , self.color1 .to_string());
        map.insert("color2" , self.color2 .to_string());
        map.insert("color3" , self.color3 .to_string());
        map.insert("color4" , self.color4 .to_string());
        map.insert("color5" , self.color5 .to_string());
        map.insert("color6" , self.color6 .to_string());
        map.insert("color7" , self.color7 .to_string());
        map.insert("color8" , self.color8 .to_string());
        map.insert("color9" , self.color9 .to_string());
        map.insert("color10", self.color10.to_string());
        map.insert("color11", self.color11.to_string());
        map.insert("color12", self.color12.to_string());
        map.insert("color13", self.color13.to_string());
        map.insert("color14", self.color14.to_string());
        map.insert("color15", self.color15.to_string());
        map.insert("foreground", self.foreground.to_string());
        map.insert("background", self.background.to_string());
        map
    }
}
