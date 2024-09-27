use crate::backends::*;


/// Requires more tweaking and more in depth testing, but seems to do the work.
/// TODO Investigate what are the better default properties that get the most average and tasteful palette.
/// `palette` `as_components()` and `components_as()` is very interesting, since it works on primitive types, need more reading.
/// from: https://github.com/okaneco/kmeans-colors/blob/master/src/bin/kmeans_colors/app.rs
pub fn kmeans(f: &Path) -> Result<Vec<u8>> {
    use kmeans_colors::{
        get_kmeans,
        get_kmeans_hamerly,
        Kmeans,
        MapColor,
        // Calculate,
        // Sort,
    };
    use palette::{
        cast::{AsComponents, ComponentsAs},
        Srgb,
    };
    //use palette::{white_point::D65, FromColor, IntoColor, Lab, LinSrgba, Srgb, Srgba};

    let img = image::ImageReader::open(f)?.with_guessed_format()?.decode()?.into_rgb8();

    // let img = thumb::thumb(f)?;

    //let img_vec: &[Srgba<u8>] = img.components_as();
    let img_vec: &[Srgb<u8>] = img.components_as();

    let k = 8;
    let max_iter = 20;
    let runs = 5;
    let verbose = false;
    let converge = 0.0025;
    let seed = 0;

    // Read image buffer into Srgb format
     let rgb_pixels = img_vec
         .iter()
    //     .filter(|x| x.alpha == 255) //only use non-transparent colors
         .map(|x| x.into_format())
         .collect::<Vec<Srgb<f32>>>();

    //TODO what's the difference between these?
    let method = if k > 1 { get_kmeans_hamerly } else { get_kmeans };

    // Iterate over amount of runs keeping best results
    let mut result = Kmeans::new();

    for i in 0..runs {
        let run_result = method(
            k,
            max_iter,
            converge,
            verbose,
            &rgb_pixels,
            seed + i,
        );

        if result.score > run_result.score {
            result = run_result;
        }
    }

    // Pre-convert centroids into output format
    let centroids = &result
        .centroids
        .iter()
        .map(|x| x.into_format())
        .collect::<Vec<Srgb<u8>>>();

    let rgb: Vec<Srgb<u8>> = Srgb::map_indices_to_centroids(centroids, &result.indices);

    Ok(
        rgb.as_components().to_vec()
    )
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
