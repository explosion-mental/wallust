use crate::backends::*;
use std::num::NonZeroU32;

use fast_image_resize as fir;

/// Resize it, then get read the image
//TODO don't resize if image is X by X large
pub fn fast_resize(f: &Path) -> Result<Vec<u8>> {
    let (true_w, true_h) = image::image_dimensions(f)?;

    let def = NonZeroU32::new(512).expect("NON ZERO");
    let w = NonZeroU32::new(true_w / 4).unwrap_or(def);
    let h = NonZeroU32::new(true_h / 4).unwrap_or(def);


    //source
    let img = image::open(f)?;
    let src_image = fir::Image::from_vec_u8(
        NonZeroU32::new(true_w).unwrap_or(def),
        NonZeroU32::new(true_h).unwrap_or(def),
        img.to_rgba8().into_raw(),
        fir::PixelType::U8,
    )?;

    //destination
    let mut dst_image = fir::Image::new(
        w,
        h,
        src_image.pixel_type(),
    );


    //resize
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Nearest);
    resizer.resize(&src_image.view(), &mut dst_image.view_mut())?;

    //alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    // Read source image from file
    //let img = ImageReader::open("./data/nasa-4928x3279.png")
     //   .unwrap()
     //   .decode()
     //   .unwrap();

    Ok(dst_image.into_vec())
}

