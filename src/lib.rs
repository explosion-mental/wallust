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

    let mut top = vec![];
    let mut orig = vec![];

    // Here we start with true so it runs at least once.
    let mut warn = true;

    if c.threshold.is_none() || dynamic_th { // automatically handled by wallust.

        //TODO one could use a binary tree split (and even maybe async) to find out the "best threshold"


        // if warn is true it means there is a problem and it requires to call 'fallback_generator'
        // when false, there was nothing wrong.

        // counter
        let mut i = 1;

        // Default error
        let mut err = colorspaces::ColorSpaceError::NotEnough;

        //TODO if no warn (meaning the first `c.color_space.main` call ran without issues)
        //do the opposite of what's below: increase the threshold until warn is true

        while warn {
            let newth = c.true_th - i;
            // println!("HEEEY {warn} and th is {newth}");

            // TODO maybe split ColorSpace::main into more steps so this call has less usage
            match c.color_space.main(&rgb8s, newth, &c.fallback_generator.unwrap_or_default(), &c.palette.sort_ord()) {
                Ok(o) => ((top, orig), warn) = o,
                // overwrite error
                Err(e) => err = e,
            }

            i += 1;

            // While this case MAY.. be possible, who knows really, we add a simple "non loop forever" exit.
            // This should be 'impossible' since `colorspaces` module checks if at least two colors are there.
            if newth == 1 { anyhow::bail!("{err}") }
        }

        if top.is_empty() { anyhow::bail!("{err}") }
    } else {
        ((top, orig), warn) = c.color_space.main(&rgb8s, c.true_th, &c.fallback_generator.unwrap_or_default(), &c.palette.sort_ord())?;
    }

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
