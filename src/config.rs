use serde::*;
use std::path::Path;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::fs::File;

use crate::Histo;

use tinytemplate::TinyTemplate;
use anyhow::Result;
use anyhow::Context;

/// Representation of the toml config file `wallust.toml`
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Configurable threshold
    pub threshold: f32,
    /// `wallust` should work with or without this
    pub entry: Option<Vec<Entries>>,
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

/// 16 colors [0..=15] + background + foreground
#[derive(serde::Serialize)]
pub struct ColorsSer {
    background: String,
    foreground: String,
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
    color8: String,
    color9: String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
}

pub fn write_template(entries: Vec<Entries>, histo: &[Histo]) -> Result<()>{
    let config = shellexpand::tilde("~/.config/wallust/");
    let config = config.as_ref();

    let context = ColorsSer {
        background : histo[0].background().to_string(),
        foreground : histo[0].foreground().to_string(),
        color0 :  histo[0].to_string(),
        color1 :  histo[1].to_string(),
        color2 :  histo[2].to_string(),
        color3 :  histo[3].to_string(),
        color4 :  histo[4].to_string(),
        color5 :  histo[5].to_string(),
        color6 :  histo[6].to_string(),
        color7 :  histo[7].to_string(),
        color8 :  histo[8].to_string(),
        color9 :  histo[9].to_string(),
        color10: histo[10].to_string(),
        color11: histo[11].to_string(),
        color12: histo[12].to_string(),
        color13: histo[13].to_string(),
        color14: histo[14].to_string(),
        color15: histo[15].to_string(),
    };


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
