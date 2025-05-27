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
pub fn gen_colors(file: &std::path::Path, c: &crate::config::Config, dynamic_th: bool, cache_path: &std::path::Path, no_cache: bool) -> anyhow::Result<crate::colors::Colors> {

    let gen = &c.fallback_generator.unwrap_or_default();
    let ord = &c.palette.sort_ord();
    let dynamic = if c.threshold.is_some() && !dynamic_th { false } else { true };

    let cache = cache::Cache::new(file, c, cache_path)?;
    use cache::IsCached as C;
    //TODO add warnings

    println!("{:?}", cache.is_cached_all());

    match cache.is_cached_all() {
        C::BackendnCSnPalette => { // (cache)Palette -> Done
            let mut colors = cache.read_palette()?;
            postcolor(c, &mut colors);
            Ok(colors)
        },
        C::BackendnCS => { // (cached)CS -> Palette -> Done
            let (top, orig, _warn) = cache.read_cs()?;
            let mut colors = c.palette.run(top, orig);
            if !no_cache { cache.write_palette(&colors)? } //COLORS
            postcolor(c, &mut colors);
            Ok(colors)
        },
        C::Backend => { // (cached)Backend -> CS -> Palette -> Done
            let rgb8s = cache.read_backend()?;
            if !no_cache { cache.write_backend(&rgb8s)? } //BACKEND

            let cs = match c.color_space.run(dynamic, &rgb8s, c.threshold.unwrap_or_default(), gen, ord) {
                Some(s) => s,
                None => anyhow::bail!("Not enough colors!"),
            };

            let (ref top, ref orig, _warn) = cs;
            if !no_cache { cache.write_cs(&cs)? } //COLORSPACE

            let mut colors = c.palette.run(top.to_vec(), orig.to_vec());
            if !no_cache { cache.write_palette(&colors)? } //COLORS
            postcolor(c, &mut colors);
            Ok(colors)
        },
        C::None => { // Generate Backend from scratch => CS -> Palette -> Done.
            let rgb8s = c.backend.main()(file)?;
            if !no_cache { cache.write_backend(&rgb8s)? } //BACKEND

            let cs = match c.color_space.run(dynamic, &rgb8s, c.threshold.unwrap_or_default(), gen, ord) {
                Some(s) => s,
                None => anyhow::bail!("Not enough colors!"),
            };

            let (ref top, ref orig, _warn) = cs;
            if !no_cache { cache.write_cs(&cs)? } //COLORSPACE

            let mut colors = c.palette.run(top.to_vec(), orig.to_vec());
            if !no_cache { cache.write_palette(&colors)? } //COLORS
            postcolor(c, &mut colors);
            Ok(colors)
        },
    }
}

pub fn postcolor(c: &crate::config::Config, colors: &mut crate::colors::Colors) {
    if c.check_contrast.unwrap_or(false) {
        colors.check_contrast_all();
    }

    if let Some(s) = c.saturation {
        colors.saturate_colors(f32::from(s) / 100.0);
    }
}
