use crate::backends::*;

/// Resize it, then get read the image
pub fn resized(f: &PathBuf, threshold: u32) -> Result<Colors<MyLab>> {
    let (true_w, true_h) = image::image_dimensions(f)?;
    let w = true_w / 4;
    let h = true_h / 4;
    let img = image::open(f)?.resize(w, h, image::imageops::Gaussian);
    let img = img.to_rgba8();

    let labs = lab::rgb_bytes_to_labs(img.as_raw());
    let mut histo = gen_histogram(labs, threshold);
    Ok(Colors::from(&mut histo))
}

