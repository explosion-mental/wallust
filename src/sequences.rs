//! How to handle color sequences for terminal emulators
use std::fs::File;
use std::path::Path;
use std::io::Write;

use palette::Srgb;
use anyhow::Result;
use owo_colors::OwoColorize;
use serde::{Serialize, Deserialize};

use crate::colors::SrgbString;
use crate::colors::Colors;
use crate::args::Sequences;

/// private fn that returns sequences
/// "Convert a hex color to a text color sequence"
pub fn set_color(color: &Srgb, index: u32) -> String {
    let s = color.strsrgb();

    format!("\x1B]4;{index};{s}\x1B\\")
}

/// Convert a hex color to a special sequence.
/// Currently no alpha is supported. The sequence below is only supported by urxvt, by pywal
pub fn set_special(color: &Srgb, index: u32) -> String {

    // if (11..=708).contains(&index) && alpha != 100 {
    //     return format!("\x1B]{index};[{alpha}]{self}\x1B\\");
    // }

    format!("\x1B]{index};{}\x1B\\", color.strsrgb())
}

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
/// Source: <https://goo.gl/KcoQgP>
/// 10 = foreground, 11 = background, 12 = cursor foreground, 13 = mouse foreground,
/// 708 = background border color.
/// ## Format
/// Escape sequences is "\033]4;%s;%s\033\\" but hex, note the escaped backslash at the end.
/// A triple `\\\` is needed to remove the new line and print a single `\`
#[cfg(target_family = "unix")]
pub fn unix_term(c: &Colors, cache_path: &Path, remove: Option<&[Sequences]>) -> Result<()> {
    let seq_file = cache_path.join("wallust/sequences");

    let sequences = c.to_seq(remove);

    // set iterm on mac
    #[cfg(target_os = "macos")]
    let sequences = sequences + &set_iterm_tab_color(c);

    #[cfg(target_os = "macos")]
    let tty_pattern = "/dev/ttys00[0-9]*";

    #[cfg(not(target_os = "macos"))]
    let tty_pattern = "/dev/pts/[0-9]*";

    // set custom devices on bsd
    #[cfg(target_os = "openbsd")]
    let devices = openbsd_ttys()?;

    // usually at /dev/pts/*
    #[cfg(not(target_os = "openbsd"))]
    let devices = glob::glob(tty_pattern).expect("glob pattern is ok");

    for entry in devices {
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
            Err(e) => anyhow::bail!("Error while sending sequences to terminals:\n{e}"),
        };
    }

    File::create(seq_file)?
        .write_all(sequences.as_bytes())?;

    Ok(())
}

/// Sets terminal colors on OpenBSD.
/// Calls `ps -o tty | sed -e 1d -e s#^#/dev/# | sort | uniq`
/// ref: <https://github.com/dylanaraps/pywal/pull/510>
#[cfg(target_os = "openbsd")]
use std::path::PathBuf;
#[cfg(target_os = "openbsd")]
fn openbsd_ttys() -> Result<Vec<Result<PathBuf>>> {
    use itertools::Itertools;
    use std::process::{Command, Stdio};
    use std::str;

    let ps = Command::new("ps").arg("-o").arg("tty")
        .stdout(Stdio::piped())       // of which we will pipe the output.
        .spawn()?;

    let ps_out = match ps.stdout {
        Some(s) => s,
        //return empty vec, to avoid quitting on an error.
        None => return Ok(vec![]),
    };

    let sed = Command::new("sed").args(["-e", "1d", "-e", "s#^#/dev/#"])
        .stdin(Stdio::from(ps_out)) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()?;

    let sort = Command::new("sort")
        .stdin(Stdio::from(sed.stdout.expect("should be filled"))) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()?;

    let mut paths = vec![];

    let output = sort.wait_with_output()?;

    //add every line
    for line in str::from_utf8(&output.stdout)?.lines().unique() {
        let p = PathBuf::try_from(line).map_err(anyhow::Error::from);
        paths.push(p);
    }

    Ok(paths)
}


const SCHEME_NAME: &str = "wallust";

/// searches for `settings.json` file to change the scheme in windows cli
#[cfg(target_family = "windows")]
pub fn windows_term(cols: &Colors) -> Result<()> {
    let Some(dir) = dirs::data_local_dir() else {
        anyhow::bail!("Couldn't get %LOCALAPPDATA%, quitting..");
    };

    let stable  = dir.join("Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json");
    let preview = dir.join("Packages/Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe/LocalState/settings.json");
    let unpkg   = dir.join("Microsoft/WindowsTerminal/settings.json");

    let files = [stable, preview, unpkg];

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

use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WinTerm {
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
struct WinScheme {
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
