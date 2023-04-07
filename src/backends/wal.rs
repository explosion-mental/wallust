use crate::backends::*;
use std::process::Command;

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
    let output = im.stdout;

    // The output is like the following:
    //   0,0: (92,64,54)  #5C4036  srgb(36.1282%,25.1188%,21.1559%)
    //            ^
    //    we care bout this representation
    for line in String::from_utf8(output)?.lines().skip(1) {
        println!("{}", line);
        let mut s = line.split_ascii_whitespace();
        let Some(_) = s.next() else {
            anyhow::bail!("convert failed giving colors");
        };
        let c = s.next().unwrap().replace(&['(', ')'], "");
        let mut split = c.split(",");
        let r = split.next().unwrap().parse::<u8>()?;
        let g = split.next().unwrap().parse::<u8>()?;
        let b = split.next().unwrap().parse::<u8>()?;
        cols.push(r);
        cols.push(g);
        cols.push(b);
    }
    //println!("{:?}", cols);
    Ok(cols)
}
