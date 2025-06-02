use crate::backends::*;

//use std::cmp::Ordering;
use std::path::Path;
use palette::{Lab, Srgb, FromColor, IntoColor, cast::{AsComponents, ComponentsAs}};
use kmeans_colors::{get_kmeans_hamerly, get_kmeans, MapColor};


const K: u8 = crate::colorspaces::MIN_COLS;

/// Requires more tweaking and more in depth testing, but seems to do the work.
/// TODO Investigate what are the better default properties that get the most average and tasteful palette.
/// `palette` `as_components()` and `components_as()` is very interesting, since it works on primitive types, need more reading.
/// from: https://github.com/okaneco/kmeans-colors/blob/master/src/bin/kmeans_colors/app.rs
pub fn kmeans(f: &Path) -> Result<Vec<u8>> {
    let img = image::ImageReader::open(f)?.with_guessed_format()?.decode()?.into_rgb8();
    // let img = super::fast_resize::fast_resize(f)?;

    // Get RGB pixels
    let img_vec: &[Srgb<u8>] = img.components_as();

    // Convert RGB -> Lab for perceptual clustering
    let pixels: Vec<Lab> = img_vec
        .iter()
        .map(|px| px.into_format::<f32>().into_color())
        .collect();

    //prefer max iter over runs.
    let max_iter = 300;
    let converge = 1e-5;
    //let runs = 1;
    let verbose = false;

    fastrand::seed(0xBEEF);

    //let mut best_result = Kmeans::new();

    let method = if K > 1 { get_kmeans_hamerly } else { get_kmeans };

    //for _ in 0..runs {
        let result = method(
            K.into(),
            max_iter,
            converge,
            verbose,
            &pixels,
            fastrand::u64(..),
        );

    //     println!("score: {}", run_result.score);
    //
    //     if best_result.score > run_result.score {
    //         best_result = run_result;
    //     }
    // }

    // Convert Lab → Srgb<f32> → Srgb<u8> (using your preferred method)
    let centroids: Vec<Srgb<u8>> = result
        .centroids
        .iter()
        .map(|&lab| Srgb::from_color(lab).into_format::<u8>())
        .collect();

    // map pixels to their nearest centroid
    let rgb: Vec<Srgb<u8>> = Srgb::map_indices_to_centroids(&centroids, &result.indices);

    Ok(rgb.as_components().to_vec())
}




// This implementation gets extremly good results. Requires +nightly tho.
// pub fn kmeans(f: &Path) -> Result<Vec<u8>> {
//     use ::kmeans::{Kmeans, KmeansConfig};
//
//     let n = 8;
//
//     // An image buffer of one black pixel and one white pixel
//     let img = image::io::Reader::open(f)?.with_guessed_format()?.decode()?;
//
//     let (w, h) = img.dimensions();
//     let data = img.into_rgb8().iter().map(|&x| f32::from(x)).collect::<Vec<f32>>();
//
//     let k = KMeans::new(data, (w * h) as usize, 3);
//     let result = k.kmeans_lloyd(n, 100, KMeans::init_kmeanplusplus, &KMeansConfig::default());
//
//     Ok(
//         result.centroids.iter().map(|x| *x as u8).collect::<Vec<u8>>()
//     )
//
// }
