#![allow(clippy::type_complexity)]
#![allow(clippy::useless_conversion)]
//! wallust - Generate a colorscheme based on an image
pub mod args;
pub mod backends;
pub mod cache;
pub mod colors;
pub mod colorspaces;
pub mod config;
pub mod palettes;
pub mod template;
pub mod themes;
pub mod sequences;

/// How [`crate::colors::Colors`] is filled, returns the colors itself and a bool that indicates whether
/// [`backends`] had some warnings or not (ugly workaround ik)
pub fn gen_colors(file: &std::path::Path, c: &crate::config::Config, dynamic_th: bool) -> anyhow::Result<(crate::colors::Colors, bool)> {
    // read image as raw rgb8 vecs
    let rgb8s = c.backend.main()(file)?;

    // get the top 16 most used colors, ordered from the darkest to lightest.
    // Different color spaces can be used here.
    // let ((mut top, mut orig), mut warn) = c.color_space.main(&rgb8s, c.true_th, &c.fallback_generator.unwrap_or_default(), &c.palette.sort_ord())?;

    // Here we start with true so it runs at least once.
    let warn;

    // let mix = c.color_space.mixed();
    // let dedup = c.color_space.to_dedup();
    let gen = &c.fallback_generator.unwrap_or_default();
    let ord = &c.palette.sort_ord();
    //let mytype = c.color_space.testy();



    let (top, orig) = if c.threshold.is_some() && !dynamic_th {
        let threshold = c.threshold.expect("checked above");
        match c.color_space.run_one(&rgb8s, threshold, gen, ord) {
            Some(s) => {
                let (t, o, w) = s;
                warn = w;
                (t, o)
            },
            None => anyhow::bail!("Not enough colors!."),
        }
    } else {
        let dummy_threshold = 0;
        match c.color_space.run_dynamic(&rgb8s, dummy_threshold, gen, ord) {
            Some(s) => {
                let (t, o, w) = s;
                warn = w;
                (t, o)
            },
            None => anyhow::bail!("Not enough colors!."),
        }
    };


    // Apply a [`Palette`] that returns the [`Colors`] struct
    let mut colors = c.palette.run(top, orig);

    if c.check_contrast.unwrap_or(false) {
        colors.check_contrast_all();
    }

    if let Some(s) = c.saturation {
        colors.saturate_colors(f32::from(s) / 100.0);
    }

    Ok((colors, warn))
}
