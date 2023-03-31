use crate::backends::*;

/// By default return all values from an image
pub fn full(f: &PathBuf, threshold: u32) -> Result<Colors<MyLab>> {
    // Init image, then convert it into rgb and finally to LAB
    let img = ImageReader::open(f)?.decode()?.to_rgba8();
    let labs = lab::rgb_bytes_to_labs(img.as_raw());

    let mut histo = gen_histogram(labs, threshold);
    Ok(Colors::from(&mut histo))
}
