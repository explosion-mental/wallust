//! wallust - Generate a colorscheme based on an image

// Only below files/modules are allowed to use `use crate::` in between them.
pub mod backends;
pub mod colors;
pub mod colorspaces;
pub mod filters;
pub mod themes;

use colors::Colors;

/// How [`Colors`] is filled, returns the colors itself and a bool that indicates whether
/// [`backends`] had some warnings or not (ugly workaround ik)
/// TODO proper errors
pub fn gen_colors(
    file: &std::path::Path,
    backend: &backends::Backend,
    color_space: colorspaces::ColorSpaces,
    threshold: u8,
    generation_mode: &colorspaces::Generate,
    filter: &filters::Filters,
    check_contrast: bool,
    saturation: Option<u8>,
    ) -> anyhow::Result<(Colors, bool)>
{
    // read image as raw rgb8 vecs
    let rgb8s = backends::main(backend)(file)?;

    // get the top 16 most used colors, ordered from the darkest to lightest. Different color
    // spaces can be used here.
    let (mut top, warn) = colorspaces::main(color_space, &rgb8s, threshold, generation_mode)?;

    // custom sorting, checkout [`ColorOrder`] and [`sort_ord`]
    top.sort_colors(&filters::sort_ord(filter));

    // Apply a [`Filters`] that returns the [`Colors`] struct
    let mut colors = filters::main(filter)(top);

    if check_contrast {
        colors.check_contrast_all();
    }

    if let Some(s) = saturation {
        colors.saturate_colors(f32::from(s) / 100.0);
    }

    Ok((colors, warn))
}
