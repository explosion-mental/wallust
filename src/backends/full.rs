use crate::backends::*;

/// By default return all values from an image
pub fn full(f: &PathBuf) -> Result<Vec<u8>> {
    // Init image, then convert it into rgb and finally to LAB
    let img = ImageReader::open(f)?.decode()?.to_rgb8();
    Ok(img.into_raw())
}
