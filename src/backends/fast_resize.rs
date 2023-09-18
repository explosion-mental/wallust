use crate::backends::*;
use std::num::NonZeroU32;

use fast_image_resize as fir;

/// Resize it, then get read the image, with an optimized algorithm that uses SIMD operations.
pub fn fast_resize(f: &Path) -> Result<Vec<u8>> {
    let (true_w, true_h) = image::image_dimensions(f)?;

    let shrink = |x| if x > 512 { x / 4 } else { x };

    let def_w = NonZeroU32::new(1024).expect("NON ZERO");
    let def_h = NonZeroU32::new(768).expect("NON ZERO");
    let w = NonZeroU32::new(shrink(true_w)).unwrap_or(def_w);
    let h = NonZeroU32::new(shrink(true_h)).unwrap_or(def_h);

    //read the image
    let img = image::open(f)?;

    // source image
    let src = fir::Image::from_vec_u8(
        NonZeroU32::new(true_w).unwrap_or(def_w),
        NonZeroU32::new(true_h).unwrap_or(def_h),
        img.into_rgba8().into_raw(),
        fir::PixelType::U8,
    )?;

    //destination (where to write new resized image)
    let mut dest = fir::Image::new(
        w,
        h,
        src.pixel_type(),
    );

    //resize
    fir::Resizer::new(fir::ResizeAlg::Nearest)
        .resize(&src.view(), &mut dest.view_mut())?;

    Ok(dest.into_vec())
}

