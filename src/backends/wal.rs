//! # Wal
//! * Uses image magick to generate the colors
//! * We parse the hex string because the tuples seems to change, like if there are no green and
//!   blue values and only red, the output would be like `(238)`, instead of `(238, 0, 0)`
//! ## Sample output of `convert` is like the following:
//! ```txt
//!   0,0: (92,64,54)  #5C4036  srgb(36.1282%,25.1188%,21.1559%)
//!   skip   skip         ^
//!                we care bout this one
//! ```
use crate::backends::*;
use crate::colors::HexConversion;
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
        cols.append(&mut hex.decode_hex()?);
    }
    Ok(cols)
}
