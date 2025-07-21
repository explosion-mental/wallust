//! # Wal
//! * Uses image magick to generate the colors
//! * We parse the hex string because the tuples seems to change, like if there are no green and
//!   blue values and only red, the output would be like `(238)`, instead of `(238, 0, 0)`
//! ## Sample output of `convert` is like the following:
//! ```txt
//!   0,0: (92,64,54)  #5C4036  srgb(36.1282%,25.1188%,21.1559%)
//!   skip      ^
//!       we care bout this one
//! ```
use crate::backends::*;
use std::process::Command;
use std::str;
use palette::Srgb;
use palette::cast::AsComponents;
use std::fmt;

enum IMcmd {
    Magick,
    Convert,
}

impl fmt::Display for IMcmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IMcmd::Magick => write!(f, "magick"),
            IMcmd::Convert => write!(f, "convert"),
        }
    }
}


/// Inspired by how pywal uses Image Magick :)
pub fn wal(f: &Path) -> Result<Vec<u8>> {
    let mut cols: Vec<Srgb<u8>> = Vec::with_capacity(16); // there will be no more than 16 colors

    let magick_command = has_im()?.to_string();

    let mut raw_colors = String::new();

    // we start with 1, since we already 'did' an iteration by initializing the variable.
    for i in 0..20 {
        raw_colors = imagemagick(16 + i, f, &magick_command)?;

        if raw_colors.lines().count() > 16 { break }

        if i == 19 {
            anyhow::bail!("Imagemagick couldn't generate a suitable palette.");
        }
        // else {
            // No need to print, just keep trying.
            // eprintln!("Imagemagick couldn't generate a palette.");
            // eprintln!("Trying a larger palette size {}", 16 + i);
        // }
    }

    for line in raw_colors.lines().skip(1) {
        let mut s = line.split_ascii_whitespace().skip(1);
        let hex = s.next().expect("Should always be present, without spaces in between e.g. (0,0,0)");
        //let hex : Srgb<u8> = *hex.parse::<Srgba<u8>>()?.into_format::<u8, u8>();
        let hex = &hex[1..hex.len() - 1];
        let rgbs: Vec<u8> = hex
                                .split(',')
                                .map(|x| x.parse::<u8>().expect("Should be a number"))
                                .collect();
        let hex = Srgb::new(rgbs[0], rgbs[1], rgbs[2]);
        cols.push(hex);
    }

    Ok(cols.as_components().to_vec())
}

fn imagemagick(color_count: u8, img: &Path, magick_command: &str) -> Result<String> {
    let im = Command::new(magick_command)
        .args([
            &format!("{}[0]", img.display()), // gif edge case, use the first frame
            "-resize", "25%",
            "-colors", &color_count.to_string(),
            "-unique-colors",
            "-colorspace", "srgb", //srgb
            "-depth", "8", // 8 bit
            "txt:-",
        ])
        .output()
        .expect("This should run, given that `has_im()` should fail first, unless IM flags are deprecated.");

    Ok(str::from_utf8(&im.stdout)?.to_owned())
}

///whether to use `magick` or good old `convert`
fn has_im() -> Result<IMcmd> {
    let m = "magick";
    let c = "convert";

    // .output() is used to 'eat' the output, instead of .spawn()
    match Command::new(&m).output() {
        Ok(_) => Ok(IMcmd::Magick),
        Err(e) => {
            match Command::new(&c).output() {
                Ok(_) => Ok(IMcmd::Convert),
                Err(e2) => Err(anyhow::anyhow!("Neither `magick` nor `convert` is invokable:\n{e} {e2}")),
            }
            // if let std::io::ErrorKind::NotFound = e.kind() {
            //     Ok("convert".to_owned())
            // } else {
            //     Err(anyhow::anyhow!("An error ocurred while executing magick: {e}"))
            // }
        },
    }
}
