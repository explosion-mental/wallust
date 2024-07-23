use crate::backends::*;

/// faster algo than the `resized` module. Hardcoded to 512x512, ignores aspect ratio
pub fn thumb(f: &Path) -> Result<Vec<u8>> {
    let img = image::ImageReader::open(f)?
        .with_guessed_format()?
        .decode()?
        .thumbnail(512, 512);
    Ok(img.into_rgb8().into_raw())
}

