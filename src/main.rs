//! wallust - Generate a colorscheme based on an image
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;
use spinners::{Spinner, Spinners};

mod args;
mod backends;
mod cache;
mod colors;
mod colorspaces;
mod config;
mod filters;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let conf = config::Config::new()?;
    let info = "I".blue().bold().to_string();

    // print some info that's gonna be used
    if ! cli.quiet {
        println!("[{info}] {img}: {f}", f = cli.file.display(), img = "image".magenta().bold());
        conf.print();
    }

    // generate hash cache file name and cache dir to either read or write to it
    let cached_data = cache::Cache::new(&cli.file, &conf)?;

    // Whether to load data from cache or to generate one from scratch
    let colors = if cached_data.is_cached() {
        if ! cli.quiet { println!("[{info}] {c}: Using cache {}", cached_data.path.italic(), c = "cache".magenta().bold()); }
        cached_data.read()?
    } else { // generate colors

        if ! cli.quiet {
            let mut sp = Spinner::with_timer(Spinners::Pong, "Generating color scheme..".into());
            match gen_colors(&cli.file, &conf) {
                Ok((o, w)) => {
                    let warn = if w {
                        format!("[{}] Not enough colors in the image, artificially generating new colors..", "W".red().bold())
                    } else {
                        "".into()
                    };

                    sp.stop_with_message(format!("{warn}[{info}] Color scheme palette generated!"));
                    o
                }
                Err(e) => {
                    sp.stop_with_message("".into());
                    return Err(e);
                },
            }
        } else {
            let (c, _) = gen_colors(&cli.file, &conf)?;
            c
        }
    };

    if ! cli.quiet {
        //TODO add print_long to list `value: color` like
        colors.print();
    }

    // Set sequences
    if ! cli.skip_sequences {
        if ! cli.quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
        colors.sequences()?;
    }

    // write entries `[[entry]]` of the config file (if any)
    if let Some(s) = conf.entry {
        if ! cli.quiet { println!("[{info}] {}: Writing templates..", "templates".magenta().bold()); }
        config::write_template(&s, &colors, cli.quiet)?
    }

    // Cache colors
    if ! cached_data.is_cached() {
        if ! cli.quiet { println!("[{info}] {}: Saving scheme to cache.", "cache".magenta().bold()); }
        cached_data.write(&colors)?;
    }

    if ! cli.quiet { colors.done(); }

    Ok(())
}

/// How [`Colors`] is filled, returns the colors itself and a bool that indicates whether
/// [`backends`] had some warnings or not (ugly workaround ik)
fn gen_colors(file: &PathBuf, c: &config::Config) -> Result<(colors::Colors, bool)> {
    // choose how to sort colors, more on [`ColorOrder`]
    let sort_ord = filters::sort_ord(&c.filter);

    // read image as raw rgb8 vecs
    let rgb8s = backends::main(&c.backend)(file)?;

    // get the top 16 most used colors, ordered from the darkest to lightest. Different color
    // spaces can be used here.
    let (top, warn) = colorspaces::main(c.color_space, &rgb8s, c.threshold, sort_ord)?;

    // Apply a [`Filters`] that returns the [`Colors`] struct
    let colors = filters::main(&c.filter)(&top);

    Ok((colors, warn))
}
