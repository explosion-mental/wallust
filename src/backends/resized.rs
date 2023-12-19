use crate::backends::*;

/// Resize it, then get read the image
pub fn resized(f: &Path) -> Result<Vec<u8>> {
    let (true_w, true_h) = image::image_dimensions(f)?;

    let (w, h) = shrink(true_w, true_h);
    let img = image::open(f)?.resize(w, h, image::imageops::Gaussian);
    Ok(img.into_rgb8().into_raw())
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
