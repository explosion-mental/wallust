//! # Wal
//! * Uses image magick to generate the colors
//! * We parse the hex string because the tuples seems to change, like if there are no green and
//!   blue values and only red, the output would be like `(238)`, instead of `(238, 0, 0)`
//! ## Sample output of `convert` is like the following:
//! ```
//!   0,0: (92,64,54)  #5C4036  srgb(36.1282%,25.1188%,21.1559%)
//!   skip   skip         ^
//!                we care bout this one
//! ```

use crate::backends::*;
use std::process::Command;
use std::str;

/// use Image Magick to get colors
//TODO flatten the hues like pywal
pub fn wal(f: &PathBuf) -> Result<Vec<u8>> {

    let im = Command::new("convert")
        .arg(f)
        .arg("-resize")
        .arg("25%")
        .arg("-colors")
        .arg("16")
        .arg("-unique-colors")
        .arg("txt:-")
        .output()?;

    let mut cols: Vec<u8> = vec![];

    for line in str::from_utf8(&im.stdout)?.lines().skip(1) {
        let mut s = line.split_ascii_whitespace().skip(2);
        let hex = s.next().expect("Should always be present e.g. #EEEEEE");
        cols.append(&mut decode_hex(hex)?);
    }
    Ok(cols)
}

/// Simple hex decode from string, input is like `#EEEEEE`
/// ref: <https://stackoverflow.com/a/52992629>
fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let s = &s[1..];
    if s.len() % 2 != 0 {
        anyhow::bail!("Error decoding hex, OddLength");
    } else {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
            .collect()
    }
}
