use crate::backends::*;

/// Resize it, then get read the image
//TODO don't resize if image is X by X large
pub fn resized(f: &Path) -> Result<Vec<u8>> {
    let (true_w, true_h) = image::image_dimensions(f)?;
    let w = true_w / 4;
    let h = true_h / 4;
    let img = image::open(f)?.resize(w, h, image::imageops::Gaussian);
    Ok(img.to_rgb8().into_raw())
}

