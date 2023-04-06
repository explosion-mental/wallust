use crate::backends::*;

/// Resize it, then get read the image
pub fn resized(f: &PathBuf) -> Result<Vec<u8>> {
    let (true_w, true_h) = image::image_dimensions(f)?;
    let w = true_w / 4;
    let h = true_h / 4;
    let img = image::open(f)?.resize(w, h, image::imageops::Gaussian);
    Ok(img.to_rgba8().into_raw())
}

