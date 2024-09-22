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

    // Read image as raw rgb8 vecs
    let rgb8s = c.backend.main()(file)?;

    let gen = &c.fallback_generator.unwrap_or_default();
    let ord = &c.palette.sort_ord();
    let dynamic = if c.threshold.is_some() && !dynamic_th { false } else { true };

    // Get the top 16 most used colors, ordered from the darkest to lightest.
    // Different color spaces can be used here.
    let (top, orig, warn) = match c.color_space.run(dynamic, &rgb8s, c.threshold.unwrap_or_default(), gen, ord) {
            Some(s) => s,
            None => anyhow::bail!("Not enough colors!"),
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
