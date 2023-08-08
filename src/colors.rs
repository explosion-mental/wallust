//! # Colors logic
//! Here [`Colors`] and [`Myrgb`] types are defined. These are simple enough used by backends,
//! colorspace and filters modules as a reference, rather than to keep using `Vec<u8>`. This way
//! the base has more structure (also because it's only 16 colors).
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use owo_colors::{OwoColorize, Rgb};
use serde::{Serialize, Deserialize};

/// This is how the scheme it's organized
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Colors {
    pub background: Myrgb,
    pub foreground: Myrgb,
    pub color0 : Myrgb,
    pub color1 : Myrgb,
    pub color2 : Myrgb,
    pub color3 : Myrgb,
    pub color4 : Myrgb,
    pub color5 : Myrgb,
    pub color6 : Myrgb,
    pub color7 : Myrgb,
    pub color8 : Myrgb,
    pub color9 : Myrgb,
    pub color10: Myrgb,
    pub color11: Myrgb,
    pub color12: Myrgb,
    pub color13: Myrgb,
    pub color14: Myrgb,
    pub color15: Myrgb,
}

/// Type that every backend should return
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Myrgb(pub u8, pub u8, pub u8);

/// Display [`Myrgb`] like hex (e.g. `(238, 238, 238)` as `#EEEEEE`)
impl fmt::Display for Myrgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

/// methods for [`Myrgb`] darken and lighten are basically from pywal `util.py` (just 'type safe' :p)
impl Myrgb {
    /// to owo [`Rgb`]
    pub fn col(&self) -> Rgb {
        Rgb(self.0, self.1, self.2)
    }

    /// darkens rgb by amount (lossy)
    pub fn darken(&self, amount: f32) -> Self {
        Self(
            (f32::from(self.0) * (1.0 - amount)) as u8,
            (f32::from(self.1) * (1.0 - amount)) as u8,
            (f32::from(self.2) * (1.0 - amount)) as u8,
        )
    }

    /// ligthen rgb by amount (lossy)
    pub fn lighten(&self, amount: f32) -> Self {
        Self(
            (f32::from(self.0) + f32::from(255 - self.0) * amount) as u8,
            (f32::from(self.1) + f32::from(255 - self.1) * amount) as u8,
            (f32::from(self.2) + f32::from(255 - self.2) * amount) as u8,
        )
    }

    /// Mix with other [`Myrgb`]
    pub fn blend(&self, other: Self) -> Self {
        Self(
            (0.5 * f32::from(self.0) + 0.5 * f32::from(other.0)) as u8,
            (0.5 * f32::from(self.1) + 0.5 * f32::from(other.1)) as u8,
            (0.5 * f32::from(self.2) + 0.5 * f32::from(other.2)) as u8,

        )
    }

    //This outputs `235,235,235` as r,g,b
    pub fn rgb(&self) -> String {
        format!("{},{},{}", self.0, self.1, self.2)
    }

    //TODO alpha
    //.rgba output `235,235,235,1.0`
    pub fn rgba(&self) -> String {
        let alpha = 1.0;
        format!("rgba({},{},{},{alpha})", self.0, self.1, self.2)
    }

    //xrgba outputs `ee/ee/ee/ff` as r/g/b/alpha in hex but using `/` as a separator
    pub fn xrgba(&self) -> String {
        format!("{:02x}/{:02x}/{:02x}/ff", self.0, self.1, self.2)
    }

    //This only "strips" the `#` from the usual output, leaving the following: `EEEEEE`
    pub fn strip(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    pub fn red(&self) -> String {
        format!("{}", self.0)
    }

    pub fn green(&self) -> String {
        format!("{}", self.1)
    }

    pub fn blue(&self) -> String {
        format!("{}", self.2)
    }
}

impl Colors {
    /// Print the scheme out
    pub fn print(&self) {
        print!(
"
{}{}{}{}{}{}{}{}
{}{}{}{}{}{}{}{}

",
        "    ".on_color(self.color0.col()),
        "    ".on_color(self.color1.col()),
        "    ".on_color(self.color2.col()),
        "    ".on_color(self.color3.col()),
        "    ".on_color(self.color4.col()),
        "    ".on_color(self.color5.col()),
        "    ".on_color(self.color6.col()),
        "    ".on_color(self.color7.col()),
        "    ".on_color(self.color8.col()),
        "    ".on_color(self.color9.col()),
        "    ".on_color(self.color10.col()),
        "    ".on_color(self.color11.col()),
        "    ".on_color(self.color12.col()),
        "    ".on_color(self.color13.col()),
        "    ".on_color(self.color14.col()),
        "    ".on_color(self.color15.col()),
        );
    }

    /// Fancy `enjoy the palette!` message
    pub fn done(&self) {
        let space = "  ".strikethrough();
        print!(
"
{}{}{}{}{}{space}{}{}{}{space}{}{}{}{}{}{}{}{}
",
        "E ".color(self.color15.col()).bold().blink(),
        "N ".color(self.color14.col()).bold().blink(),
        "J ".color(self.color13.col()).bold().blink(),
        "O ".color(self.color12.col()).bold().blink(),
        "Y ".color(self.color11.col()).bold().blink(),
        "T ".color(self.color10.col()).bold().blink(),
        "H ".color(self.color9 .col()).bold().blink(),
        "E ".color(self.color8 .col()).bold().blink(),
        "P ".color(self.color7 .col()).bold().blink(),
        "A ".color(self.color6 .col()).bold().blink(),
        "L ".color(self.color5 .col()).bold().blink(),
        "E ".color(self.color4 .col()).bold().blink(),
        "T ".color(self.color3 .col()).bold().blink(),
        "T ".color(self.color2 .col()).bold().blink(),
        "E ".color(self.color1 .col()).bold().blink(),
        "! ".color(self.color0 .col()).bold().blink(),
        );
    }

    /// Sets terminal colors
    /// ANSI escape codes tables and helpful guidelines:
    /// <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797>
    ///
    /// TODO investigate about iTerm2 (macOS/Darwin)
    pub fn sequences(&self, cache_path: &Path) -> anyhow::Result<()> {
        #[cfg(target_family = "windows")]
        return windows_term(self);

        #[cfg(target_family = "unix")]
        return unix_term(self, cache_path);
    }
}

use serde_json::Value;

/// Set iTerm2 tab/window color
/// `\a` is BELL in octal escape byte, `\x07` in hex
#[cfg(target_os = "macos")]
fn set_iterm_tab_color(c: &Colors) -> String {
    let col = c.background.rgb();
    format!(
"\x1B]6;1;bg;red;brightness;{col}\x07\\\
\x1B]6;1;bg;green;brightness;{col}\x07\\\
\x1B]6;1;bg;blue;brightness;{col}\x07\\\
"
    )
}

/// Uses terminal sequences to update terminal colors
/// ref: <https://github.com/dylanaraps/pywal/blob/master/pywal/sequences.py>
/// ## Special colors.
/// Source: https://goo.gl/KcoQgP
/// 10 = foreground, 11 = background, 12 = cursor foreground, 13 = mouse foreground,
/// 708 = background border color.
/// ## Format
/// Escape sequences is "\033]4;%s;%s\033\\" but hex, note the escaped backslash at the end.
/// A triple `\\\` is needed to remove the new line and print a single `\`.
#[cfg(target_family = "unix")]
fn unix_term(c: &Colors, cache_path: &Path) -> Result<()> {
    let seq_file = cache_path.display().to_string() + "/wallust/sequences";

    #[cfg(target_os = "macos")]
    let iterm = set_iterm_tab_color(c);

    #[cfg(target_os = "macos")]
    let tty_pattern = "/dev/ttys00[0-9]*";

    #[cfg(not(target_os = "macos"))]
    let tty_pattern = "/dev/pts/[0-9]*";

    #[cfg(not(target_os = "macos"))]
    let iterm = "";

    let sequences = format!(
"\x1B]4;0;{col0}\x1B\\\
\x1B]4;1;{col1}\x1B\\\
\x1B]4;2;{col2}\x1B\\\
\x1B]4;3;{col3}\x1B\\\
\x1B]4;4;{col4}\x1B\\\
\x1B]4;5;{col5}\x1B\\\
\x1B]4;6;{col6}\x1B\\\
\x1B]4;7;{col7}\x1B\\\
\x1B]4;8;{col8}\x1B\\\
\x1B]4;9;{col9}\x1B\\\
\x1B]4;10;{col10}\x1B\\\
\x1B]4;11;{col11}\x1B\\\
\x1B]4;12;{col12}\x1B\\\
\x1B]4;13;{col13}\x1B\\\
\x1B]4;14;{col14}\x1B\\\
\x1B]4;15;{col15}\x1B\\\
\x1B]10;{fg}\x1B\\\
\x1B]11;{bg}\x1B\\\
\x1B]12;{cursor}\x1B\\\
\x1B]13;{fg}\x1B\\\
\x1B]17;{fg}\x1B\\\
\x1B]19;{bg}\x1B\\\
\x1B]4;232;{bg}\x1B\\\
\x1B]4;256;{fg}\x1B\\\
\x1B]4;257;{bg}\x1B{iterm}\\\
",
    bg = c.background,
    fg = c.foreground,
    cursor = c.foreground,
    col0  = c.color0,
    col1  = c.color1,
    col2  = c.color2,
    col3  = c.color3,
    col4  = c.color4,
    col5  = c.color5,
    col6  = c.color6,
    col7  = c.color7,
    col8  = c.color8,
    col9  = c.color9,
    col10 = c.color10,
    col11 = c.color11,
    col12 = c.color12,
    col13 = c.color13,
    col14 = c.color14,
    col15 = c.color15,
    );

    for entry in glob::glob(tty_pattern).expect("glob pattern is ok") {
        match entry {
            Ok(path) => {
                match File::create(&path) {
                    Ok(o) => o,
                    Err(e) => { //ignore errors, but report them
                        eprintln!("[{w}] Couldn't write to {p}: {e}", p = path.display(), w = "W".red().bold());
                        continue;
                    },
                }.write_all(sequences.as_bytes())?
            },
            Err(e) => {
                anyhow::bail!("Error while sending sequences to terminals:\n{e}")
            },
        };
    }

    File::create(seq_file)?
        .write_all(sequences.as_bytes())?;

    Ok(())
}

const SCHEME_NAME: &str = "wallust";

/// searches for `settings.json` file to change the scheme in windows cli
#[cfg(target_family = "windows")]
fn windows_term(cols: &Colors) -> Result<()> {
    let Some(dir) = dirs::data_local_dir() else {
        anyhow::bail!("Couldn't get %LOCALAPPDATA%, quitting..");
    };

    let stable  = dir.join("Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json");
    let preview = dir.join("Packages/Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe/LocalState/settings.json");
    let unpkg   = dir.join("Microsoft/WindowsTerminal/settings.json");

    let files = vec![stable, preview, unpkg];

    for i in files {
        let content = match std::fs::read_to_string(&i) {
            Ok(o) => o,
            Err(_) => continue,
        };

        let mut settings_json = match serde_json::from_str::<WinTerm>(&content) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[{w}] Deserializing json failed {p}: {e}", p = i.display(), w = "W".red().bold());
                continue;
            }
        };

        let mut found = false;

        for (i, s) in settings_json.schemes.iter().enumerate() {
            if s.name == SCHEME_NAME {
                settings_json.schemes[i] = cols.into();
                found = true;
                break; //only do this once, it should only be one "wal" scheme anyway
            }
        }

        // a "wallust" scheme wasn't found, append it.
        if found == false {
            settings_json.schemes.push(cols.into());
        }

        let new_json = match serde_json::to_string_pretty(&settings_json) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[{w}] Writing json failed {p}: {e}", p = i.display(), w = "W".red().bold());
                continue;
            }
        };

        File::create(&i)?
            .write_all(new_json.as_bytes())?
    }

    Ok(())

}

impl From<&Colors> for WinScheme {
    fn from(c: &Colors) -> Self {
        Self {
            name                : SCHEME_NAME.to_string(),
            cursor_color        : c.color8 .to_string(),
            selection_background: c.color15.to_string(),
            foreground          : c.foreground.to_string(),
            background          : c.background.to_string(),
            black               : c.color0 .to_string(),
            blue                : c.color4 .to_string(),
            cyan                : c.color5 .to_string(),
            green               : c.color1 .to_string(),
            purple              : c.color2 .to_string(),
            red                 : c.color3 .to_string(),
            white               : c.color15.to_string(),
            yellow              : c.color6 .to_string(),
            bright_black        : c.color8 .to_string(),
            bright_blue         : c.color12.to_string(),
            bright_cyan         : c.color13.to_string(),
            bright_green        : c.color9 .to_string(),
            bright_purple       : c.color10.to_string(),
            bright_red          : c.color11.to_string(),
            bright_white        : c.color7 .to_string(),
            bright_yellow       : c.color14.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinTerm {
    #[serde(rename = "$help")]
    pub help: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    pub actions: Value,
    pub copy_formatting: String,
    pub copy_on_select: bool,
    pub default_profile: String,
    pub new_tab_menu: Value,
    pub profiles: Value,
    pub themes: Value,
    /// This is the only field we need
    pub schemes: Vec<WinScheme>,
}

/// a WindowsTerminal Scheme
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WinScheme {
    pub name: String,
    pub cursor_color: String,
    pub selection_background: String,
    pub background: String,
    pub foreground: String,
    pub black: String,
    pub blue: String,
    pub cyan: String,
    pub green: String,
    pub purple: String,
    pub red: String,
    pub white: String,
    pub yellow: String,
    pub bright_black: String,
    pub bright_blue: String,
    pub bright_cyan: String,
    pub bright_green: String,
    pub bright_purple: String,
    pub bright_red: String,
    pub bright_white: String,
    pub bright_yellow: String,
}

pub trait HexConversion {
    fn decode_hex(&self) -> Result<Vec<u8>>;
}

/// Simple hex decode from string
/// * input `#EEEEEE` or `EEEEEE`
/// * output `[238, 238, 238]`
/// ref: <https://stackoverflow.com/a/52992629>
/// # Example
/// ```
/// # use wallust::colors::HexConversion;
/// let gray    = "#EEEEEE".decode_hex().unwrap();
/// let no_hash = "EE0000".decode_hex().unwrap();
/// assert_eq!(vec![238, 238, 238], gray);
/// assert_eq!(vec![238, 0, 0], no_hash);
/// ```
///
/// # Errors
/// ```
/// # use wallust::colors::HexConversion;
/// let wrong_letter   = "#EEEEEG".decode_hex().unwrap_err();
/// let unneeded_chars = "##EEEEEE".decode_hex().unwrap_err();
/// assert_eq!("invalid digit found in string", wrong_letter.to_string());
/// assert_eq!("Error decoding hex, OddLength", unneeded_chars.to_string());
/// ```
impl HexConversion for &str {
    fn decode_hex(&self) -> Result<Vec<u8>> {
        let s = if &self[..1] == "#" { &self[1..] } else { self };
        let len = s.len();

        if len % 2 != 0 {
            anyhow::bail!("Error decoding hex, OddLength");
        } else {
            (0..len)
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
                .collect()
        }
    }
}

/// From a vec to Myrgb
impl From<Vec<u8>> for Myrgb {
    fn from(v: Vec<u8>) -> Myrgb {
        Myrgb(v[0], v[1], v[2])
    }
}
