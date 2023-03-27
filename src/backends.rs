//! # Backends
//! There are multiple methods in which you can get the most relevant colors from an image; rather
//! than hardcoding, give options
use std::path::PathBuf;


use crate::delta::delta_e;
use crate::{MyLab, Colors, Histo};

use image::io::Reader as ImageReader;
use anyhow::Result;
use lab::Lab;

/// Threshold to accept the color difference
/// This is temporary, this constant should be auto to get the best result depending on the image
/// size (XXX maybe a threshold for image size then?)
//const TH: u32 = 20;

/// By default return all values from an image
pub fn full(f: &PathBuf, threshold: u32) -> Result<Colors<MyLab>> {
    // Init image, then convert it into rgb and finally to LAB
    let img = ImageReader::open(f)?.decode()?.to_rgba8();
    let labs = lab::rgb_bytes_to_labs(img.as_raw());

    let mut histo: Vec<Histo> = vec![];

    for lab in labs {
        if is_present(lab, &mut histo, threshold) {
            continue;
        } else {
            histo.push(Histo { color: lab, count: 1 });
        }

        //let a = lab.to_rgb();
        //print!("{}   ", "COLOR".on_color(Rgb(a[0], a[1], a[2])));
    }

    // sort vec by count
    histo.sort_by(|a, b| b.count.cmp(&a.count));
    Ok(Colors::from(&histo))

    //darken the Lab color. Maybe apply these in [`is_present`] ?
    //for i in &mut histo {
    //    i.darken(0.5);
    //}
}

/// Resize it, then get read the image
pub fn resized(f: &PathBuf, threshold: u32) -> Result<Colors<MyLab>> {
    let (true_w, true_h) = image::image_dimensions(f)?;
    let w = true_w / 4;
    let h = true_h / 4;
    let img = image::open(f)?.resize(w, h, image::imageops::Gaussian);
    let img = img.to_rgba8();

    let labs = lab::rgb_bytes_to_labs(img.as_raw());

    let mut histo: Vec<Histo> = vec![];

    for lab in labs {
        if is_present(lab, &mut histo, threshold) {
            continue;
        } else {
            histo.push(Histo { color: lab, count: 1 });
        }

        //let a = lab.to_rgb();
        //print!("{}   ", "COLOR".on_color(Rgb(a[0], a[1], a[2])));
    }

    // sort vec by count
    histo.sort_by(|a, b| b.count.cmp(&a.count));
    Ok(Colors::from(&histo))
}

/// determines whether a Lab color is present in our histogram, by using [`delta_e`] we compare if
/// colors are similar enough, using the [`TH`] (threshold)
fn is_present(color: Lab, histogram: &mut Vec<Histo>, threshold: u32) -> bool {
    for e in histogram {
        // if any lab value is between a threshold, count it up
        if delta_e(color, e.color) < threshold {
            e.mix(color);
            e.count += 1;
            return true;
        }
    }
    false
}
