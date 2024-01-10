use crate::backends::*;


/// Requires more tweaking and more in depth testing, but seems to do the work.
/// TODO Investigate what are the better default properties that get the most average and tasteful palette.
/// from: https://github.com/okaneco/kmeans-colors/blob/master/src/bin/kmeans_colors/app.rs
#[allow(unused)]
pub fn kmeans(f: &Path) -> Result<Vec<u8>> {


    use kmeans_colors::{get_kmeans, get_kmeans_hamerly, Calculate, Kmeans, MapColor, Sort};
    use palette::cast::{AsComponents, ComponentsAs};
    use palette::{white_point::D65, FromColor, IntoColor, Lab, LinSrgba, Srgb, Srgba};
    use image::GenericImageView;
    use rand::Rng;

    struct OPT {
        k: u8,
        max_iter: usize,
        runs: usize,
        verbose: bool,
    }

    // An image buffer of one black pixel and one white pixel
    let img = image::io::Reader::open(f)?.with_guessed_format()?.decode()?.into_rgba8();

    let converge = 0.0025;
    //let seed: u64 = rand::thread_rng().gen();
    let seed = 12345;

    // TODO skip srgba and go directly to rgb, ignoring transparency.
    let img_vec: &[Srgba<u8>] = img.components_as();

    let opt = OPT {
        k: 8,
        max_iter: 20,
        runs: 3,
        verbose: false,
    };

    // Read image buffer into Srgb format
    let rgb_pixels: Vec<Srgb<f32>> = img_vec
        .iter()
        .filter(|x| x.alpha == 255) //only use non-transparent colors
        .map(|x| Srgb::<f32>::from_color(x.into_format::<_, f32>()))
        .collect();

    //TODO what's the difference between these?
    let method = if opt.k > 1 { get_kmeans_hamerly } else { get_kmeans };

    // Iterate over amount of runs keeping best results
    let mut result = Kmeans::new();
    //TODO check these fields in detail
    for i in 0..opt.runs {
        let run_result = method(
            opt.k as usize,
            opt.max_iter,
            converge,
            opt.verbose,
            &rgb_pixels,
            seed + i as u64,
        );

        if run_result.score < result.score {
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
