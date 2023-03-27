//! # Backends
//! There are multiple methods in which you can get the most relevant colors from an image; rather
//! than hardcoding, give options
use std::path::PathBuf;

use image::io::Reader as ImageReader;
use anyhow::Result;
use lab::Lab;

pub fn parse_image(filename: PathBuf) -> Result<Vec<Lab>> {
    default(&filename)
}

/// By default return all values from an image
fn default(f: &PathBuf) -> Result<Vec<Lab>> {
    // Init image, then convert it into rgb and finally to LAB
    let img = ImageReader::open(f)?.decode()?.to_rgba8();
    Ok(lab::rgb_bytes_to_labs(img.as_raw()))
}

/// Resize it, then get read the image
fn resize(f: &PathBuf) -> Result<Vec<Lab>> {
    let (true_w, true_h) = image::image_dimensions(f)?;
    let w = true_w / 4;
    let h = true_h / 4;
    let img = image::open(f)?.resize(w, h, image::imageops::Gaussian);
    let img = img.to_rgba8();
    Ok(lab::rgb_bytes_to_labs(img.as_raw()))
}
