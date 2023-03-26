//use std::io::Cursor;
use image::io::Reader as ImageReader;
use std::collections::HashMap;

//use colorsys::{ColorAlpha, Hsl, Rgb};
use lab::{Lab,rgbs_to_labs};
use owo_colors::*;

//TODO use BTree from std instead of HashMap
//TODO handle errors
//TODO clap

struct Histo {
    value: Lab,
    count: usize,
}

/// threshold to accept the color difference
const TH: f32 = 10.0;

/// #definition thingy
/// > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100. The a*
/// > axis is relative to the green–red opponent colors, with negative values toward green and positive
/// > values toward red. The b* axis represents the blue–yellow opponents, with negative numbers toward
/// > blue and positive toward yellow.
/// ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = "./test.png";

    // Init image, then convert it into rgb and finally to LAB
    let img = ImageReader::open(file)?.decode()?.to_rgba8();
    let rgbs = img.as_raw();
    let labs = lab::rgb_bytes_to_labs(rgbs);
    //let rgbs = img.to_rgb8().get_pixel::<>();

    //      Lab, count,
    let mut histo: Vec<Histo> = vec![];

    for lab in labs {
        if is_present(lab, &mut histo) {
            continue;
        } else {
            histo.push(Histo {value: lab, count: 1});
        }

        //let a = lab.to_rgb();
        //print!("{}   ", "COLOR".on_color(Rgb(a[0], a[1], a[2])));
    }

    for i in histo {
        let a = i.value.to_rgb();
        println!("{} x {} times", "COLOR".on_color(Rgb(a[0], a[1], a[2])), i.count);
    }

    Ok(())
}

/// This should use delta_e somehow
fn is_present(lab: Lab, v: &mut Vec<Histo>) -> bool {
    for i in v {
        // if any lab value is between a threshold, count it up
        if delta_e(lab, i.value) <= TH {
            i.count += 1;
            return true;
        }
    }
    false
}

/// Returns how much the colors differ
///
/// ref: <https://www.easyrgb.com/en/math.php>
fn delta_e(current: Lab, previous: Lab) -> f32 {
        let deltae =
            (
                (
                        (((previous.l - current.l) as i32) ^ 2)
                    +   (((previous.a - current.a) as i32) ^ 2)
                    +   (((previous.b - current.b) as i32) ^ 2)
                )
            as f32).sqrt();

        deltae
}
