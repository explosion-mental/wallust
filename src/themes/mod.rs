//! # Themes
//! These module should be able to read terminal sexy templates, as well as the pywal colorschemes.
//! Other formats could be added if needed and requested. A compiletime feature is used to `mod`
//! and `use` the `colorschemes.rs` module and [`built_in_theme()`] function.
//! For reading external colorschemes: `wallust cs my_colorscheme.json` \n
//! For using the built in themes: `wallust theme zenburn` \n
use std::fmt;
use std::path::Path;

use crate::colors::{Colors, HexConversion};

use anyhow::Result;
use serde::Deserialize;

#[cfg(feature = "themes")]
use colorschemes::COLS_VALUE;
#[cfg(feature = "themes")]
pub use colorschemes::COLS_KEY;

#[cfg(feature = "themes")]
pub mod colorschemes;

#[derive(Deserialize)]
pub struct WalSpecial {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct WalTheme {
    pub special: WalSpecial,
    pub colors: WalColors,
}

/// Terminal-Sexy format
#[derive(Deserialize)]
pub struct TerminalSexy {
    pub name: String,
    pub author: String,
    pub color: Vec<String>,
    pub foreground: String,
    pub background: String,
}

/// Possible formats to read from
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Schemes {
    /// uses the wal colorscheme format,
    /// see <https://github.com/dylanaraps/pywal/tree/master/pywal/colorschemes>
    Pywal,
    /// uses <https://terminal.sexy> JSON export
    TerminalSexy,
}

/// reads a $file with an specified $format
pub fn read_scheme(file: &Path, format: Schemes) -> Result<Colors> {
    let contents = std::fs::read_to_string(file)?;

    match format {
        Schemes::Pywal => {
            let ser: WalTheme = serde_json::from_str(&contents)?;
            ser.to_colors()

        },
        Schemes::TerminalSexy => {
            let ser: TerminalSexy = serde_json::from_str(&contents)?;
            ser.to_colors()
        },
    }
}

use owo_colors::{OwoColorize, AnsiColors};

/// Try all possible [`Schemes`] for the file
pub fn try_all_schemes(file: &Path) -> Result<Colors> {
    let contents = std::fs::read_to_string(file)?;
    let ser: Result<WalTheme, serde_json::Error> = serde_json::from_str(&contents);
    let info = "I".blue().bold().to_string();
    let cs = "colorscheme format".magenta().bold().to_string();

    match ser {
        Ok(o) => {
            let s = Schemes::Pywal;
            println!("[{info}] {cs}: Using {}", s.to_string().to_ascii_lowercase().color(s.col()));
            o.to_colors()
        },
        Err(_) => {
            let mut errs: Vec<String> = vec![];
            let warn = "W".red().bold().to_string();
            errs.push(format!("[{warn}] {} is not in `pywal` format.", file.display()));

            let ser: Result<TerminalSexy, serde_json::Error> = serde_json::from_str(&contents);
            match ser {
                Ok(o) => {
                    let s = Schemes::TerminalSexy;
                    println!("[{info}] {cs}: Using {}", s.to_string().to_ascii_lowercase().color(s.col()));
                    o.to_colors()
                },
                Err(_) => {
                    errs.push(format!("[{warn}] {} is not in `terminal-sexy` format.", file.display()));
                    for i in errs {
                        eprintln!("{i}");
                    }
                    anyhow::bail!("{} was not in the pywal or terminal-sexy format.", file.display())
                },
            }
        }
    }
}

#[cfg(feature = "themes")]
#[test]
fn keys_to_values_match() {
    for i in COLS_KEY {
        built_in_theme(i.to_string(), true).expect("{i} should find a match");
    }
}

/// Use the built in themes. STATIC Data from [`COLS_VALUE`] should be correct, which are in json
/// [`WalTheme`] format
#[cfg(feature = "themes")]
pub fn built_in_theme(theme_key: String, quiet: bool) -> Result<Colors> {
    use rand::Rng;

    let index = if theme_key == "random" {
        let i = rand::thread_rng().gen_range(0..=COLS_VALUE.len() - 1); //ommit the last item, which is "random"
        if ! quiet { println!("[{info}] {theme}: randomly selected {name}", theme = "theme".magenta().bold(), name = COLS_KEY[i], info = "I".blue().bold()); }
        Some(i)
    } else {
        COLS_KEY.iter().position(|&x| x == theme_key)
    };

    match index {
        Some(s) => serde_json::from_str::<WalTheme>(COLS_VALUE[s]).expect("json format MUST be correct").to_colors(),
        None => anyhow::bail!("Theme not found. Quitting..."),
    }
}

impl Schemes {
    pub fn col(&self) -> AnsiColors {
        match self {
            Schemes::Pywal => AnsiColors::Blue,
            Schemes::TerminalSexy => AnsiColors::Magenta,
        }
    }
}

/// Add a simple `Display` for [`Schemes`]
impl fmt::Display for Schemes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Schemes::Pywal => write!(f, "Pywal"),
            Schemes::TerminalSexy => write!(f, "Terminal-Sexy"),
        }
    }
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

impl TerminalSexy {
    fn to_colors(&self) -> Result<Colors> {
        let c = &self.color;
        let fg = &self.foreground;
        let bg = &self.background;

        Ok(
            Colors {
                background: bg.as_str().decode_hex()?.into(),
                foreground: fg.as_str().decode_hex()?.into(),
                color0 : c[0].as_str().decode_hex()?.into(),
                color1 : c[1].as_str().decode_hex()?.into(),
                color2 : c[2].as_str().decode_hex()?.into(),
                color3 : c[3].as_str().decode_hex()?.into(),
                color4 : c[4].as_str().decode_hex()?.into(),
                color5 : c[5].as_str().decode_hex()?.into(),
                color6 : c[6].as_str().decode_hex()?.into(),
                color7 : c[7].as_str().decode_hex()?.into(),
                color8 : c[8].as_str().decode_hex()?.into(),
                color9 : c[9].as_str().decode_hex()?.into(),
                color10: c[10].as_str().decode_hex()?.into(),
                color11: c[11].as_str().decode_hex()?.into(),
                color12: c[12].as_str().decode_hex()?.into(),
                color13: c[13].as_str().decode_hex()?.into(),
                color14: c[14].as_str().decode_hex()?.into(),
                color15: c[15].as_str().decode_hex()?.into(),
            }
        )
    }
}
