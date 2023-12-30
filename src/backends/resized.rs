use image::GenericImageView;

use crate::backends::*;

/// Resize it, then get read the image
pub fn resized(f: &Path) -> Result<Vec<u8>> {
    let img = image::io::Reader::open(f)?
        .with_guessed_format()?
        .decode()?;

    let (true_w, true_h) = img.dimensions();

    let (w, h) = shrink(true_w, true_h);

    Ok(
        img
        .resize(w, h, image::imageops::Gaussian)
        .into_rgb8()
        .into_raw()
    )
}

/// Calculates the new resized sizes, **keeping the aspect ratio**.
fn shrink(w: u32, h: u32) -> (u32, u32) {
    let resized = if w >= 1024 || h >= 1024 { true } else { false };

    if resized {
        (w / 2, h / 2)
    } else {
        (w, h)
    }
}
