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
        .arg("9")
        .arg("-unique-colors")
        .arg("txt:-")
        .output()?;

    let mut cols: Vec<u8> = vec![];

    // Sample output of `convert` is like the following:
    //   0,0: (92,64,54)  #5C4036  srgb(36.1282%,25.1188%,21.1559%)
    //            ^
    //    we care bout this one

    for line in str::from_utf8(&im.stdout)?.lines().skip(1) {
        let mut s = line.split_ascii_whitespace().skip(1);

        let c = s.next().expect("Should be present as e.g. (0, 0, 0)").replace(['(', ')'], "");
        let mut split = c.split(',');

        let r = split.next().expect("convert outputs this").parse::<u8>()?;
        let g = split.next().expect("convert outputs this").parse::<u8>()?;
        let b = split.next().expect("convert outputs this").parse::<u8>()?;

        cols.push(r);
        cols.push(g);
        cols.push(b);
    }
    //println!("{:?}", cols);
    Ok(cols)
}
