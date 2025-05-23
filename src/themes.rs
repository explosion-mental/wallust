//! # Themes
//! These module should be able to read terminal sexy templates, as well as the pywal colorschemes.
//! Other formats could be added if needed and requested. A compiletime feature is used to `mod`
//! and `use` the `colorschemes.rs` module and [`built_in_theme()`] function.
//! For reading external colorschemes: `wallust cs my_colorscheme.json` \n
//! For using the built in themes: `wallust theme zenburn` \n
//! Functions and const values are tested.
use std::fmt;
use std::path::Path;

use crate::colors::Colors;

use anyhow::Result;
use serde::Deserialize;

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

use glob::glob;
use crate::config::WalStr;
/// First, it reads files, so the user could "overwrite" built in themes with theirs.
/// The order in which this works is the following.
/// 1. Looks up a file on `wallust/colorschemes`:
/// 2. First, it will look for an exact match.
/// 3. If one isn't found, it will admit file extensions, if there is one match.
/// 4. If there are multiple files with the same name, but different file extensions, error out and exit.
/// 5. If none, or more than one, file matches, searches if a theme name matches.
pub fn search_theme_or_cs(name: &str, quiet: bool, confpath: &Path, schemes: Option<Schemes>) -> Result<(WalStr, Colors)> {
    //#1 wallust/colorschemes/
    let p = confpath.join("colorschemes").join(name);
    //println!("{p:?} and | NAME {name}");

    // #2 exact match
    if p.exists() { return Ok((WalStr::Path(p.clone()), try_all_schemes(&p, quiet)?)); }
    // #3 accept file extensions with wildcard '*'
    let myglob = glob(&format!("{}*", p.display())).expect("glob pattern is ok");

    let count = myglob.count();

    if count == 1 { //#3
        for i in glob(&format!("{}*", p.display())).expect("glob pattern is ok") {
            match i {
                Ok(o) => {
                    let cs = match schemes {
                        Some(s) => read_scheme(&o, &s),
                        None => try_all_schemes(&o, quiet),
                    };
                    return Ok((WalStr::Path(o), cs?))
                },
                Err(e) => anyhow::bail!("Found match for '{name}', but could not opened: {e}"),
            }
        }
        anyhow::bail!("Should be unreacheable");
    } else if count == 0 { // #5
        match built_in_theme(name, quiet) {
            Some(s) => return Ok((WalStr::Theme(name.to_owned()), s)),
            None => anyhow::bail!("No matches for '{name}'.") //it's not a built in theme
        }
    } else { // #4
        anyhow::bail!("There are many matches for '{name}', consider renaming.") //it's not a built in theme
    }
}


pub fn read_scheme(f: &Path, format: &Schemes) -> Result<Colors> {
    let contents = std::fs::read_to_string(f)?;
    deser_scheme(&contents, format)

}

/// deserialize the contents from a file
fn deser_scheme(contents: &str, format: &Schemes) -> Result<Colors> {
    match format {
        Schemes::Pywal => {
            let ser: WalTheme = serde_json::from_str(contents)?;
            ser.to_colors()

        },
        Schemes::TerminalSexy => {
            let ser: TerminalSexy = serde_json::from_str(contents)?;
            ser.to_colors()
        },
        Schemes::Wallust => {
            let ser: Colors = serde_json::from_str(contents)?;
            Ok(ser)
        },
    }
}

use owo_colors::{OwoColorize, AnsiColors};

/// Try all possible [`Schemes`] for the file
pub fn try_all_schemes(file: &Path, quiet: bool) -> Result<Colors> {
    let info = "I".blue().bold().to_string();
    let cs = "colorscheme format".magenta().bold().to_string();

    let a = [
        Schemes::Pywal,
        Schemes::TerminalSexy,
        Schemes::Wallust,
    ];

    let contents = std::fs::read_to_string(file)?;

    for i in &a {
        match deser_scheme(&contents, i) {
            Ok(o) => {
                if ! quiet { println!("[{info}] {cs}: Using {}", i.to_string().to_ascii_lowercase().color(i.col())); }
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
/// string that is inside the "theme" collection but acts as a keyword. The "random" theme is not a
/// theme itself, but a selected random one.
pub const RAND: &str = "random";

#[cfg(feature = "themes")]
/// string that is inside the "theme" collection but acts as a keyword. The "random" theme is not a
/// theme itself, but a selected random one.
pub const LIST: &str = "list";

#[cfg(feature = "themes")]
/// Lists all the themes
// TODO maybe use columns to display it more efficiently..
pub fn list_themes() {
    let mut cols = wallust_themes::COLS_KEY;
    cols.sort();
    let cols = cols
        .iter()
        .map(|x| format!("- {x}"))
        .collect::<Vec<_>>()
        .join("\n")
        ;

    println!("\
{themes}:
{cols}
{extra}:
- {RAND} (select a random theme)
- {LIST} (lists available themes) \
",
    extra = "Extra".bold().green(),
    themes = "Available themes".bold().green(),
    )
}

/// Use the built in themes. STATIC Data from [`COLS_VALUE`] should be correct, which are in json [`WalTheme`] format
/// If None, the theme was not found.
#[cfg(feature = "themes")]
pub fn built_in_theme(theme_key: &str, quiet: bool) -> Option<Colors> {
    use wallust_themes::COLS_KEY;
    use wallust_themes::COLS_VALUE;
    use crate::colors::Myrgb;

    let index = if theme_key == RAND {
        let i = fastrand::usize(0..COLS_KEY.len());
        if ! quiet { println!("[{info}] {theme}: randomly selected {name}", theme = "theme".magenta().bold(), name = COLS_KEY[i], info = "I".blue().bold()); }
        Some(i)
    } else {
        COLS_KEY.iter().position(|&x| x == theme_key)
    };

    match index {
        Some(s) => {
            let c = COLS_VALUE[s];
            let c = c
                .iter()
                .map(|x| {
                    let [b, g, r, _a] = x.to_le_bytes();
                    let s = Srgb::<u8>::new(r, g, b);
                    Myrgb(s.into_format())
                })
            .collect::<Vec<_>>();

            Some(
                Colors {
                    color0:  c[0],
                    color1:  c[1],
                    color2:  c[2],
                    color3:  c[3],
                    color4:  c[4],
                    color5:  c[5],
                    color6:  c[6],
                    color7:  c[7],
                    color8:  c[8],
                    color9:  c[9],
                    color10: c[10],
                    color11: c[11],
                    color12: c[12],
                    color13: c[13],
                    color14: c[14],
                    color15: c[15],
                    background: c[16],
                    foreground: c[17],
                    cursor: c[18],
                }
            )
        },
        None => None,
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

use palette::Srgb;

impl WalTheme {
    fn to_colors(&self) -> Result<Colors> {
        let c = &self.colors;
        let s = &self.special;
        Ok(
            Colors {
                cursor: s.cursor.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                background: s.background.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                foreground: s.foreground.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color0 : c.color0 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color1 : c.color1 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color2 : c.color2 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color3 : c.color3 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color4 : c.color4 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color5 : c.color5 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color6 : c.color6 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color7 : c.color7 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color8 : c.color8 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color9 : c.color9 .parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color10: c.color10.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color11: c.color11.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color12: c.color12.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color13: c.color13.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color14: c.color14.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color15: c.color15.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
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
                cursor: fg.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                background: bg.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                foreground: fg.parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color0 : c[0 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color1 : c[1 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color2 : c[2 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color3 : c[3 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color4 : c[4 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color5 : c[5 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color6 : c[6 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color7 : c[7 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color8 : c[8 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color9 : c[9 ].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color10: c[10].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color11: c[11].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color12: c[12].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color13: c[13].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color14: c[14].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
                color15: c[15].parse::<Srgb<u8>>()?.into_format::<u8>().into(),
            }
        )
    }
}
