//! # Themes
//! These module should be able to read terminal sexy templates, as well as the pywal colorschemes.
//! Other formats could be added if needed and requested. A compiletime feature is used to `mod`
//! and `use` the `colorschemes.rs` module and [`built_in_theme()`] function.
//! For reading external colorschemes: `wallust cs my_colorscheme.json` \n
//! For using the built in themes: `wallust theme zenburn` \n
//! Functions and const values are tested.
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
    /// cached wallust files
    Wallust,
}

/// reads a $file with an specified $format
pub fn read_scheme(file: &Path, format: &Schemes) -> Result<Colors> {
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
        Schemes::Wallust => {
            let ser: Colors = serde_json::from_str(&contents)?;
            Ok(ser)
        },
    }
}

use owo_colors::{OwoColorize, AnsiColors};

/// Try all possible [`Schemes`] for the file
pub fn try_all_schemes(file: &Path) -> Result<Colors> {
    let info = "I".blue().bold().to_string();
    let cs = "colorscheme format".magenta().bold().to_string();

    let a = [
        Schemes::Pywal,
        Schemes::TerminalSexy,
        Schemes::Wallust,
    ];

    for i in &a {
        match read_scheme(file, &i) {
            Ok(o) => {
                println!("[{info}] {cs}: Using {}", i.to_string().to_ascii_lowercase().color(i.col()));
                return Ok(o);
            },
            Err(_) => { continue; },
        }
    }

    //no theme found
    let (themes, last) = a.split_at(a.len() - 1);
    let themes = themes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
    anyhow::bail!("{} was not in the {themes} or {} format.", file.display(), last[0].to_string())
}

#[cfg(feature = "themes")]
#[test]
fn keys_to_values_match() {
    for i in COLS_KEY {
        if let Err(e) = built_in_theme(i.to_string(), true) {
            eprintln!("Failed {i}: {e}");
        }
    }
}

/// string that is inside the "theme" collection but acts as a keyword. The "random" theme is not a
/// theme itself, but a selected random one.
const RAND: &str = "random";

/// Use the built in themes. STATIC Data from [`COLS_VALUE`] should be correct, which are in json
/// [`WalTheme`] format
#[cfg(feature = "themes")]
pub fn built_in_theme(theme_key: String, quiet: bool) -> Result<Colors> {
    use anyhow::Context;
    use rand::Rng;

    let index = if theme_key == RAND {
        let i = rand::thread_rng().gen_range(0..=COLS_VALUE.len() - 1); //ommit the last item, which is "random"
        if ! quiet { println!("[{info}] {theme}: randomly selected {name}", theme = "theme".magenta().bold(), name = COLS_KEY[i], info = "I".blue().bold()); }
        Some(i)
    } else {
        COLS_KEY.iter().position(|&x| x == theme_key)
    };

    match index {
        Some(s) => serde_json::from_str::<WalTheme>(COLS_VALUE[s]).context(format!("using '{s}' failed")).expect("JSON should be correct").to_colors(),
        None => anyhow::bail!("Theme not found. Quitting..."),
    }
}

impl Schemes {
    pub fn col(&self) -> AnsiColors {
        match self {
            Schemes::Pywal => AnsiColors::Blue,
            Schemes::TerminalSexy => AnsiColors::Magenta,
            Schemes::Wallust => AnsiColors::Red,
        }
    }
}

/// Add a simple `Display` for [`Schemes`]
impl fmt::Display for Schemes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Schemes::Pywal => write!(f, "Pywal"),
            Schemes::TerminalSexy => write!(f, "Terminal-Sexy"),
            Schemes::Wallust => write!(f, "Wallust"),
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
