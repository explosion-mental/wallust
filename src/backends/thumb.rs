use crate::backends::*;

/// faster algo than the `resized` module. Hardcoded to 512x512
pub fn thumb(f: &PathBuf) -> Result<Vec<u8>> {
    let img = image::open(f)?.thumbnail(512, 512);
    Ok(img.to_rgb8().into_raw())
}

