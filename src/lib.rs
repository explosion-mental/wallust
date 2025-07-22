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
pub mod presets;

use std::path::Path;

use anyhow::Result;
use spinners::{Spinner, Spinners};
use owo_colors::OwoColorize;

use self::colors::Colors;
use self::colorspaces::FallbackGenerator;
use self::args::Globals;


/// Simple wrapper around spinner, to avoid allocations and the like.
//#[derive(Debug)]
pub struct SpiWrap {
    s: Option<Spinner>,
}

impl SpiWrap {
    pub fn new(quiet: bool) -> Self {
        let s = match quiet {
            false => Some(Spinner::with_timer(Spinners::Pong, "Generating color scheme..".into())),
            true => None
        };
        Self { s }
    }

    pub fn stop_warn(&mut self, gen: &FallbackGenerator) {
        if let Some(sp) = &mut self.s {
            let symbol = "[  🗸   🗸    ]";
            sp.stop_with_symbol(symbol);
            print!("[{info}] Not enough colors in the image, artificially generating new colors...\n[{info}] {method}: Using {g} to fill the palette\n",
                g = gen.to_string().color(gen.col()),
                info = "I".blue().bold(),
                method = "fallback generation method".magenta().bold()
            );
        }
    }

    pub fn stop(&mut self) {
        if let Some(sp) = &mut self.s {
            let symbol = "[    🗸    ]";
            sp.stop_with_symbol(symbol);
            print!("[{info}] Color scheme palette generated!", info = "I".blue().bold());
        }
    }
}

/// These methods are to avoid code duplication, used in main
impl Globals {
    pub fn set_seq(&self, colors: &Colors, cache_path: &Path) -> Result<()> {
        let info = "I".blue();
        let info = info.bold();
        let g = self;
        if !g.skip_sequences && !g.update_current {
            if !g.quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
            colors.sequences(cache_path, g.ignore_sequence.as_deref())?;
        }
        Ok(())
    }

    pub fn update_cur(&self, colors: &Colors) -> Result<()> {
        let info = "I".blue();
        let info = info.bold();
        let g = self;
        if g.update_current {
            if !g.quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
            print!("{}", colors.to_seq(g.ignore_sequence.as_deref()));
        }
        Ok(())
    }

}


/// How [`crate::colors::Colors`] is filled, returns the colors itself and a bool that indicates whether
/// [`backends`] had some warnings or not (ugly workaround ik)
pub fn gen_colors(file: &std::path::Path, c: &crate::config::Config, dynamic_th: bool, cache_path: &std::path::Path, no_cache: bool, quiet: bool, overwrite_cache: bool) -> anyhow::Result<crate::colors::Colors> {

    let gen = &c.fallback_generator.unwrap_or_default();
    let ord = &c.palette.sort_ord();
    let dynamic = if c.threshold.is_some() && !dynamic_th { false } else { true };

    let cache = cache::Cache::new(file, c, cache_path)?;
    use cache::IsCached as C;

    // Having to only read the schemepalette is TOO FAST to have the spinner.
    let is_cached_all = cache.is_cached_all();
    let quiet = quiet || (matches!(is_cached_all, C::BackendnCSnPalette) && !overwrite_cache);
    let mut spi = SpiWrap::new(quiet);
    // println!("{:?}", cache.is_cached_all());

    if overwrite_cache {
            let rgb8s = c.backend.main()(file)?;
            if !no_cache { cache.write_backend(&rgb8s)? } //BACKEND

            let cs = match c.color_space.run(dynamic, &rgb8s, c.threshold.unwrap_or_default(), gen, ord) {
                Some(s) => s,
                None => anyhow::bail!("Not enough colors!"),
            };

            let (ref top, ref orig, warn) = cs;
            if !no_cache { cache.write_cs(&cs)? } //COLORSPACE


            let mut colors = c.palette.run(top.to_vec(), orig.to_vec());
            if !no_cache { cache.write_palette(&colors)? } //COLORS
            postcolor(c, &mut colors);
            if warn { spi.stop_warn(gen) } else { spi.stop() }
            Ok(colors)
    } else {
        match is_cached_all {
            C::BackendnCSnPalette => { // (cache)Palette -> Done
                let mut colors = cache.read_palette()?;
                postcolor(c, &mut colors);
                spi.stop();
                if !quiet { print!("[{info}] Using cache at {}", cache.name.display(), info = "I".blue().bold()); }
                Ok(colors)
            },
            C::BackendnCS => { // (cached)CS -> Palette -> Done
                let (top, orig, warn) = cache.read_cs()?;
                let mut colors = c.palette.run(top, orig);
                if !no_cache { cache.write_palette(&colors)? } // COLORS
                postcolor(c, &mut colors);
                if warn { spi.stop_warn(gen) } else { spi.stop() }
                if !quiet { print!("[{info}] Using cache at {}", cache.name.display(), info = "I".blue().bold()); }
                Ok(colors)
            },
            C::Backend => { // (cached)Backend -> CS -> Palette -> Done
                let rgb8s = cache.read_backend()?;

                let cs = match c.color_space.run(dynamic, &rgb8s, c.threshold.unwrap_or_default(), gen, ord) {
                    Some(s) => s,
                    None => anyhow::bail!("Not enough colors!"),
                };

                let (ref top, ref orig, warn) = cs;
                if !no_cache { cache.write_cs(&cs)? } //COLORSPACE

                let mut colors = c.palette.run(top.to_vec(), orig.to_vec());
                if !no_cache { cache.write_palette(&colors)? } //COLORS
                postcolor(c, &mut colors);
                if warn { spi.stop_warn(gen); } else { spi.stop(); }
                Ok(colors)
            },
            C::Preset => {
                // TODO Cache PresetnBackend scenario
                // TODO this isn't ideal, try putting these into is_cached_all() and avoid expecting()
                let colors = if cache.preset.as_ref().expect("cache.preset: validated in is_cached_all()").exists() {
                    cache.read_preset()?
                } else {
                    let p = c.preset.as_ref().expect("Already validated that this isn't None.");
                    let rgb8s = p.backend(file)?;
                    let cs = p.cs(rgb8s);
                    p.palette(cs)
                };

                if !no_cache { cache.write_preset(&colors)? }
                spi.stop();
                Ok(colors)
            },
            C::None => { // Generate Backend from scratch => CS -> Palette -> Done.
                let rgb8s = c.backend.main()(file)?;
                if !no_cache { cache.write_backend(&rgb8s)? } //BACKEND

                let cs = match c.color_space.run(dynamic, &rgb8s, c.threshold.unwrap_or_default(), gen, ord) {
                    Some(s) => s,
                    None => anyhow::bail!("Not enough colors!"),
                };

                let (ref top, ref orig, warn) = cs;
                if !no_cache { cache.write_cs(&cs)? } //COLORSPACE

                let mut colors = c.palette.run(top.to_vec(), orig.to_vec());
                if !no_cache { cache.write_palette(&colors)? } //COLORS
                postcolor(c, &mut colors);
                if warn { spi.stop_warn(gen) } else { spi.stop() }
                Ok(colors)
            },
        }

    }
}

/// These steps are not cached, since they are variable and cheap operations. Keep the original
/// scheme in which this is done and then apply these.
pub fn postcolor(c: &crate::config::Config, colors: &mut crate::colors::Colors) {
    if c.check_contrast.unwrap_or(false) {
        colors.check_contrast_all();
    }

    if let Some(s) = c.saturation {
        colors.saturate_colors(f32::from(s) / 100.0);
    }
}
