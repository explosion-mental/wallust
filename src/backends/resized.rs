use crate::backends::*;

/// Resize it, then get read the image
pub fn resized(f: &Path) -> Result<Vec<u8>> {
    let (true_w, true_h) = image::image_dimensions(f)?;
    let shrink = |x| if x > 512 { x / 4 } else { x };
    let w = shrink(true_w);
    let h = shrink(true_h);
    let img = image::open(f)?.resize(w, h, image::imageops::Gaussian);
    Ok(img.into_rgb8().into_raw())
}

