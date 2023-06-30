//! # Themes
//! These module should be able to read terminal sexy templates, as well as the pywal colorschemes.
//! The intended use case is to simply run this the same as with an image:
//! ```shell
//! wallust gruvbox.json
//! ```
//! The current workaround requires the file to have a `.json` extension, since it only reads those anyway.
//!
//! * TODO OPTIONALLY (with compile time features) integrate the classic `pywal`/terminal sexy
//!        themes in the binary and access them with `-t/--theme` (e.g. `wallust --theme 3024`)
use std::{path::PathBuf, fs::DirEntry};

use crate::colors::{Colors, HexConversion};

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WalSpecial {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

#[derive(Serialize, Deserialize)]
pub struct WalColors {
    pub color0 : String,
    pub color1 : String,
    pub color2 : String,
    pub color3 : String,
    pub color4 : String,
    pub color5 : String,
    pub color6 : String,
    pub color7 : String,
    pub color8 : String,
    pub color9 : String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,
}

/// Pywal colorscheme
#[derive(Serialize, Deserialize)]
pub struct WalTheme {
    pub special: WalSpecial,
    pub colors: WalColors,
}

impl WalTheme {
    fn to_colors(&self) -> Result<Colors> {
        let c = &self.colors;
        let s = &self.special;
        Ok(
            Colors {
                background: s.background.as_str().decode_hex()?.into(),
                foreground: s.foreground.as_str().decode_hex()?.into(),
                color0 : c.color0.as_str().decode_hex()?.into(),
                color1 : c.color1.as_str().decode_hex()?.into(),
                color2 : c.color2.as_str().decode_hex()?.into(),
                color3 : c.color3.as_str().decode_hex()?.into(),
                color4 : c.color4.as_str().decode_hex()?.into(),
                color5 : c.color5.as_str().decode_hex()?.into(),
                color6 : c.color6.as_str().decode_hex()?.into(),
                color7 : c.color7.as_str().decode_hex()?.into(),
                color8 : c.color8.as_str().decode_hex()?.into(),
                color9 : c.color9.as_str().decode_hex()?.into(),
                color10: c.color10.as_str().decode_hex()?.into(),
                color11: c.color11.as_str().decode_hex()?.into(),
                color12: c.color12.as_str().decode_hex()?.into(),
                color13: c.color13.as_str().decode_hex()?.into(),
                color14: c.color14.as_str().decode_hex()?.into(),
                color15: c.color15.as_str().decode_hex()?.into(),
            }
        )
    }
}

pub fn wal(path: &PathBuf) -> Result<Colors> {
    let contents = std::fs::read_to_string(path)?;
    let ser: WalTheme = serde_json::from_str(&contents)?;
    ser.to_colors()
}

use std::collections::HashMap;
use include_dir::{include_dir, Dir, DirEntry as MyDir};

/// raw '[u8] from the files
static COLS_DIR: Dir<'_> = include_dir!("colorschemes/dark");

lazy_static::lazy_static! {
    /// colorschemes from files to a hashmap
    // TODO this should be a compile time feature
    static ref COLS: HashMap<String, Colors> = {
        //let p_dark = std::fs::read_dir("./colorschemes/dark/").unwrap();
        let mut ret = HashMap::new();
        let read = |x| {
            let ser: WalTheme = serde_json::from_str(x).unwrap();
            ser.to_colors().unwrap()
        };

        for i in COLS_DIR.entries() {
            let file = match i {
                MyDir::File(a) => a,
                MyDir::Dir(_) => continue,
            };
            ret.insert(
                file.path().file_stem().unwrap().to_string_lossy().to_string(),
                read(file.contents_utf8().unwrap())
            );
        }
        ret
    };
}

pub fn new(file: String) -> Result<Colors> {
    match COLS.get(&file) {
        Some(s) => Ok(*s),
        None => panic!("NO FILE"),
    }
}

// pub fn terminalsexy(path: &PathBuf) -> Result<Colors> {
//     let contents = std::fs::read_to_string(path)?;
//     Ok(serde_json::from_str(&contents)?)
// }
//
