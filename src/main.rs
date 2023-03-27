//use std::collections::HashMap;
//use std::io::Cursor;
//use colorsys::{ColorAlpha, Hsl, Rgb};

use clap::Parser;
use image::io::Reader as ImageReader;
use lab::Lab;
use anyhow::Result;

mod args;
mod config;
mod delta;
mod colors;
use delta::delta_e;
use args::Cli;
use config::*;
use colors::*;

//TODO handle errors
//TODO generate background and foreground colors, in relation to black and white
//XXX BTree?
//XXX generate an actual scheme, rather than listing colors¿

/// Threshold to accept the color difference
/// This is temporary, this constant should be auto to get the best result depending on the image
/// size (XXX maybe a threshold for image size then?)
const TH: u32 = 20;

/// #About LAB
/// > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
/// > The a* axis is relative to the green-red opponent colors, with negative values toward green
/// > and positive > values toward red.
/// > The b* axis represents the blue-yellow opponents, with negative numbers toward
/// > blue and positive toward yellow.
/// ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Init image, then convert it into rgb and finally to LAB
    let img = ImageReader::open(cli.file)?.decode()?.to_rgba8();
    let labs = lab::rgb_bytes_to_labs(img.as_raw());

    let mut histo: Vec<Histo> = vec![];

    for lab in labs {
        if is_present(lab, &mut histo) {
            continue;
        } else {
            histo.push(Histo { color: lab, count: 1 });
        }

        //let a = lab.to_rgb();
        //print!("{}   ", "COLOR".on_color(Rgb(a[0], a[1], a[2])));
    }

    // sort vec by count
    histo.sort_by(|a, b| b.count.cmp(&a.count));

    //darken the Lab color. Maybe apply these in [`is_present`] ?
    //for i in &mut histo {
    //    i.darken(0.5);
    //}

    let colors = Colors::from(&histo);

    let conf = parse_conf()?;
    match conf.entry {
        None => (),
        Some(s) => config::write_template(s, &histo)?,
    };

    colors.print();
    //TODO force 16 colors. maybe use `--theme`s, like `wal`, as backup colors
    //for i in histo {
    // only print the top 16 colors
    //println!("background:{}", histo[0].background().print_cols());
    //println!("foreground:{}", histo[0].foreground().print_cols());

    //for (i, color) in histo.iter().take(16).enumerate() {
    //    let space = if i < 10 { "    " } else { "   " };
    //    println!("color{}:{}{}", i, space, color.print_cols());
    //}

    Ok(())
}

/// determines whether a Lab color is present in our histogram, by using [`delta_e`] we compare if
/// colors are similar enough, using the [`TH`] (threshold)
fn is_present(color: Lab, histogram: &mut Vec<Histo>) -> bool {
    for e in histogram {
        // if any lab value is between a threshold, count it up
        if delta_e(color, e.color) < TH {
            e.mix(color);
            e.count += 1;
            return true;
        }
    }
    false
}
