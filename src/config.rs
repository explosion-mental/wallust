use serde::*;
use std::path::Path;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::fs::File;

use crate::Colors;
use crate::MyLab;

use tinytemplate::TinyTemplate;
use anyhow::Result;
use anyhow::Context;

/// Representation of the toml config file `wallust.toml`
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Configurable threshold
    pub parser: Parser,
    /// `wallust` should work with or without this
    pub entry: Option<Vec<Entries>>,
}

/// This indicates what 'parser' method to use, in the config file
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Parser {
    Full,
    Resized,
}

// An entry within the config file
#[derive(Debug, Deserialize)]
pub struct Entries {
    /// A file inside `~/.config/wallust/`, which is used for templating
    pub template: String,
    /// The actual path of config files
    pub path: String,
}

pub fn parse_conf() -> Result<Config> {
    let config = shellexpand::tilde("~/.config/wallust/wallust.toml");
    let config = config.as_ref();

    if ! Path::new(&config).exists() {
        panic!("no config file");
    }

    let contents = read_to_string(config)
        .with_context(|| format!("Failed to read file {}", config))?;
    let conf: Config = toml::from_str(&contents)?;
    //println!("{:#?}", conf);
    Ok(conf)
}

pub fn write_template(entries: Vec<Entries>, values: &Colors<MyLab>) -> Result<()>{
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
            (e.path, read_to_string(&path)
                        .with_context(|| format!("Failed to read file {}", path))?
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
            .with_context(|| format!("Failed to create file {}", path))?;
        buffer.write_all(rendered.as_bytes())
            .with_context(|| format!("Failed to write to file {}", path))?;
        //println!("FROM: '{path}' --- '{}'", rendered);
    }
    Ok(())
}
