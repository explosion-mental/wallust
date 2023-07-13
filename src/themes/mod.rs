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
use std::path::PathBuf;

use crate::colors::{Colors, HexConversion};

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[cfg(feature = "themes")]
use colorschemes::COLS_VALUE;
#[cfg(feature = "themes")]
pub use colorschemes::COLS_KEY;

#[cfg(feature = "themes")]
pub mod colorschemes;

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

/// Possible formats to read from
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Schemes {
    /// uses the wal colorscheme format
    /// see <https://github.com/dylanaraps/pywal/tree/master/pywal/colorschemes>
    Pywal,
    /// uses <https://terminal.sexy> JSON export
    TerminalSexy,
}

/// reads a $file with an specified $format
pub fn read_scheme(file: &PathBuf, format: Schemes) -> Result<Colors> {
    let contents = std::fs::read_to_string(file)?;

    match format {
        Schemes::Pywal => {
            let ser: WalTheme = serde_json::from_str(&contents)?;
            ser.to_colors()

        },
        Schemes::TerminalSexy => {
            todo!()
        },
    }
}

/// Try all possible [`Schemes`] for the file
// TODO finish
pub fn try_all_schemes(file: &PathBuf) -> Result<Colors> {
    let contents = std::fs::read_to_string(file)?;
    let ser: WalTheme = serde_json::from_str(&contents)?;
    ser.to_colors()
}

/// Use the built in themes. STATIC Data from [`COLS_VALUE`] should be correct.
#[cfg(feature = "themes")]
pub fn built_in_theme(theme_key: String) -> Result<Colors> {
    let mut i = 0;
    let mut found = false;

    for e in COLS_KEY {
        if e == theme_key {
            found = true;
            break;
        }
        i += 1;
    }

    if found == false {
        anyhow::bail!("Theme not found. Quitting...")
    }

    // use WalTheme, since these themes are gathered from pywal
    let ser: WalTheme = serde_json::from_str(COLS_VALUE[i]).expect("json format MUST be correct");
    ser.to_colors()
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

