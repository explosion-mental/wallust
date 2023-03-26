//use std::collections::HashMap;
//use std::io::Cursor;

//use colorsys::{ColorAlpha, Hsl, Rgb};
use clap::Parser;
use image::io::Reader as ImageReader;
use lab::Lab;
use owo_colors::*;

mod args;
use args::Cli;
//TODO handle errors
//TODO generate background and foreground colors, in relation to black and white
//XXX BTree?
//XXX generate an actual scheme, rather than listing colors¿

/// Simple Histogram
struct Histo {
    /// LAB colors
    color: Lab,
    /// number of times it has appeared
    count: usize,
}

impl Histo {
    pub fn darken(&mut self, amount: f32) {
        let lightness = self.color.l * (1.0 - amount);
        self.color.l = lightness;
    }
}

/// Threshold to accept the color difference
/// This is temporary, this constant should be auto to get the best result depending on the image
/// size (XXX maybe a threshold for image size then?)
const TH: f32 = 20.0;

/// #About LAB
/// > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100. The a*
/// > axis is relative to the green-red opponent colors, with negative values toward green and positive
/// > values toward red. The b* axis represents the blue-yellow opponents, with negative numbers toward
/// > blue and positive toward yellow.
/// ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    for i in &mut histo {
        i.darken(0.5);
    }

    //TODO force 16 colors. maybe use `--theme`s, like `wal`, as backup colors
    //for i in histo {
    // only print the top 16 colors
    for i in histo.iter().take(16) {
        let a = i.color.to_rgb();
        println!("{} x {}\t\t{:?}", "    ".on_color(Rgb(a[0], a[1], a[2])), i.count, a);
    }

    Ok(())
}

/// determines whether a Lab color is present in our histogram, by using [`delta_e`] we compare if
/// colors are similar enough, using the [`TH`] (threshold)
fn is_present(color: Lab, histogram: &mut Vec<Histo>) -> bool {
    for e in histogram {
        // if any lab value is between a threshold, count it up
        if delta_e(color, e.color) < TH {
            e.count += 1;
            return true;
        }
    }
    false
}

/// Returns how much the colors differ
///
/// ref: <https://www.easyrgb.com/en/math.php>
//XXX worth using f32?
fn delta_e(current: Lab, previous: Lab) -> f32 {
    (   ((previous.l - current.l).powf(2.0))
    +   ((previous.a - current.a).powf(2.0))
    +   ((previous.b - current.b).powf(2.0)) ).sqrt()
}
