use serde::*;
use std::path::Path;
use std::fs::read_to_string;
use crate::Histo;

use tinytemplate::TinyTemplate;


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

pub fn parse_conf() -> Config {
    let home = std::env::var("HOME").unwrap();
    let config = home + "/.config/wallust/wallust.toml";

    if ! Path::new(&config).exists() {
        panic!("no config file");
    }

    let contents = read_to_string(config).unwrap();
    let conf: Config = toml::from_str(&contents).unwrap();
    println!("{:#?}", conf);
    conf
}

#[derive(serde::Serialize)]
pub struct Context {
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

pub fn write_template(entries: Vec<Entries>, histo: &Vec<Histo>) {
    let mut tt = tinytemplate::TinyTemplate::new();
    let home = std::env::var("HOME").unwrap();
    let config = home + "/.config/wallust/";
    //let tmp = &conf.entry.unwrap()[0];

    //for e in entries {
    let template = std::fs::read_to_string(config.clone() + &entries[0].template).unwrap();
    tt.add_template("colors", &template).unwrap();

    let context = Context {
        color0 : format!("{:?}", histo[0].color),
        color1 : format!("{:?}", histo[1].color),
        color2 : format!("{:?}", histo[2].color),
        color3 : format!("{:?}", histo[3].color),
        color4 : format!("{:?}", histo[4].color),
        color5 : format!("{:?}", histo[5].color),
        color6 : format!("{:?}", histo[6].color),
        color7 : format!("{:?}", histo[7].color),
        color8 : format!("{:?}", histo[8].color),
        color9 : format!("{:?}", histo[9].color),
        color10: format!("{:?}", histo[10].color),
        color11: format!("{:?}", histo[11].color),
        color12: format!("{:?}", histo[12].color),
        color13: format!("{:?}", histo[13].color),
        color14: format!("{:?}", histo[14].color),
        color15: format!("{:?}", histo[15].color),
        color16: format!("{:?}", histo[16].color),
    };
    let rendered = tt.render("colors", &context).unwrap();
    println!("{}", rendered);
}
