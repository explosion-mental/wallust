use crate::backends::*;

/// Read and return the whole image pixels rgb8 array
pub fn full(f: &PathBuf) -> Result<Vec<u8>> {
    // Init image, then convert it into rgb and finally to LAB
    Ok(
        ImageReader::open(f)?
            .decode()?
            .to_rgb8()
            .into_raw()
    )
}
