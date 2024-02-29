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

/// How [`crate::colors::Colors`] is filled, returns the colors itself and a bool that indicates whether
/// [`backends`] had some warnings or not (ugly workaround ik)
pub fn gen_colors(file: &std::path::Path, c: &crate::config::Config) -> anyhow::Result<(crate::colors::Colors, bool)> {
    // read image as raw rgb8 vecs
    let rgb8s = backends::main(&c.backend)(file)?;

    // get the top 16 most used colors, ordered from the darkest to lightest. Different color
    // spaces can be used here.
    let ((top, orig), warn) = colorspaces::main(c.color_space, &rgb8s, c.threshold, &c.fallback_generator.unwrap_or_default(), &c.palette.sort_ord())?;

    // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
    // top = topsort_algo(&palettes::sort_ord(&c.palette));

    // Apply a [`Palette`] that returns the [`Colors`] struct
    let mut colors = palettes::main(&c.palette)(top, orig);

    if c.check_contrast.unwrap_or(false) {
        colors.check_contrast_all();
    }

    if let Some(s) = c.saturation {
        colors.saturate_colors(f32::from(s) / 100.0);
    }

    Ok((colors, warn))
}
