use serde::*;
use std::path::Path;
use std::fs::read_to_string;
use crate::Histo;

use tinytemplate::TinyTemplate;
use anyhow::Result;

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
    let home = std::env::var("HOME")?;
    let config = home + "/.config/wallust/wallust.toml";

    if ! Path::new(&config).exists() {
        panic!("no config file");
    }

    let contents = read_to_string(config)?;
    let conf: Config = toml::from_str(&contents)?;
    println!("{:#?}", conf);
    Ok(conf)
}

#[derive(serde::Serialize)]
pub struct Context {
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
    color16: String,
}

pub fn write_template(entries: Vec<Entries>, histo: &Vec<Histo>) -> Result<()>{
    let home = std::env::var("HOME")?;
    let config = home + "/.config/wallust/";

    let context = Context {
        background : format!("{}", histo[0]),
        foreground : format!("{}", histo[1]),
        color0 : format!("{}", histo[0]),
        color1 : format!("{}", histo[1]),
        color2 : format!("{}", histo[2]),
        color3 : format!("{}", histo[3]),
        color4 : format!("{}", histo[4]),
        color5 : format!("{}", histo[5]),
        color6 : format!("{}", histo[6]),
        color7 : format!("{}", histo[7]),
        color8 : format!("{}", histo[8]),
        color9 : format!("{}", histo[9]),
        color10: format!("{}", histo[10]),
        color11: format!("{}", histo[11]),
        color12: format!("{}", histo[12]),
        color13: format!("{}", histo[13]),
        color14: format!("{}", histo[14]),
        color15: format!("{}", histo[15]),
        color16: format!("{}", histo[16]),
    };


    // contents of config files
    let mut contents = vec![];

    // gather `String`s of the contents of the entries (in order to cast it down to &str)
    for e in entries {
        let path = config.clone() + &e.template;
        //println!("->'{}'", &path);
        contents.push(
            (e.path, read_to_string(path)?)
        );
    }

    let mut tt = TinyTemplate::new();

    use std::io::prelude::*;
    use std::fs::File;

    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path
    for (path, stuff) in &contents {
        tt.add_template("colors", stuff)?;
        let rendered = tt.render("colors", &context)?;
        let mut buffer = File::create(shellexpand::full(path)?.as_ref())?;
        buffer.write_all(rendered.as_bytes())?;
        //println!("FROM: '{path}' --- '{}'", rendered);
    }
    Ok(())
}
