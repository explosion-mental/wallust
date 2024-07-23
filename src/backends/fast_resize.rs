use crate::backends::*;
use fast_image_resize::images::{Image, ImageRef};
use fast_image_resize::Resizer;
use fast_image_resize::PixelType;
use image::GenericImageView;

/// Resize it, then get read the image, with an optimized algorithm that uses SIMD operations.
/// TODO for some reason this method likes really small sizes. Working with 512 or more creates
/// `green` "glitched" colors, that's why we don't use `shrink()` from `resized` module in here
pub fn fast_resize(f: &Path) -> Result<Vec<u8>> {
    //read the image and guess format
    let img = image::ImageReader::open(f)?
        .with_guessed_format()?
        .decode()?;

    let (true_w, true_h) = img.dimensions();

    //custom shrink
    let s = |x| if x > 512 { x / 4 } else { x };

    let pixels = img.into_rgb8().into_raw();

    // source image
    let src = ImageRef::new(
        true_w,
        true_h,
        &pixels,
        PixelType::U8x3, //u8 u8 u8 (r g b)
    )?;

    //destination (where to write new resized image)
    let mut dst = Image::new(
        s(true_w),
        s(true_h),
        src.pixel_type(),
    );

    // Create Resizer instance and resize source image
    // into buffer of destination image.
    let mut resizer = Resizer::new();
    // By default, Resizer multiplies and divides by alpha channel
    // images with U8x2, U8x4, U16x2 and U16x4 pixels.
    resizer.resize(&src, &mut dst, None)?;

    //resize
    // fir::Resizer::new(fir::ResizeAlg::Nearest)
    //     .resize(&src.view(), &mut dest.view_mut())?;

    Ok(dst.into_vec())
}
