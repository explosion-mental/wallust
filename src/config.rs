//! Config related stuff, like parsing the config file and writing templates defined on it
use serde::*;
use std::path::Path;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

use crate::Colors;
use crate::MyLab;

use tinytemplate::TinyTemplate;
use anyhow::Result;
use anyhow::Context;

/// Representation of the toml config file `wallust.toml`
#[derive(Debug, Deserialize)]
pub struct Config {
    /// threshold to use to differentiate colors
    pub threshold: u32,
    /// Which backend to use, see backends.rs
    pub backend: Backend,
    /// toml table with template and config target (optional)
    pub entry: Option<Vec<Entries>>,
}

/// This indicates what 'parser' method to use, in the config file
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Full,
    Resized,
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

/// Constructs a new config file
pub fn parse_conf() -> Result<Config> {
    let config = shellexpand::tilde("~/.config/wallust/wallust.toml");
    let config = config.as_ref();

    if ! Path::new(&config).exists() {
        panic!("no config file");
    }

    let contents = read_to_string(config)
        .with_context(|| format!("Failed to read file {}:\n", config))?;
    let conf: Config = toml::from_str(&contents)
        .with_context(|| format!("Failed to deserialize config file {}:\n", config))?;
    //println!("{:#?}", conf);
    Ok(conf)
}

/// Writes `template`s into `target`s
pub fn write_template(entries: &[Entries], values: &Colors<MyLab>) -> Result<()>{
    let config = shellexpand::tilde("~/.config/wallust/");
    let config = config.as_ref();

    let context: Colors<String> = Colors::from(values);

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

/// Add a simple `Display` for [`Backend`], used in main() to print which is in use
impl fmt::Display for Backend {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Full    => write!(f, "full"),
            Self::Resized => write!(f, "resized"),
        }
    }
}
