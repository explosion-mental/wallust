//! wallust - Generate a colorscheme based on an image
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;
use spinners::{Spinner, Spinners};
use std::ffi::OsStr;

use wallust::{
    args,
    backends,
    cache,
    colors,
    colorspaces,
    config,
    filters,
    template,
    themes,
};

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let info = "I".blue().bold().to_string();

    // init directories
    let Some(config_path) = dirs::config_dir() else {
        anyhow::bail!("Config path for the platform could not be found, please report this at <https://codeberg.org/explosion-mental/wallust/issues>");
    };
    let Some(cache_path) = dirs::cache_dir() else {
        anyhow::bail!("The cache path for the platform could not be found, please report this at <https://codeberg.org/explosion-mental/wallust/issues>");
    };

    // check config file or generate one if not one isn't found
    let conf = config::Config::new(&config_path)?;
    // generate hash cache file name and cache dir to either read or write to it
    let cached_data = cache::Cache::new(&cli.file, &conf, &cache_path)?;

    let is_theme = cli.file.extension().and_then(OsStr::to_str) == Some("json") || cli.theme != None;

    // print some info that's gonna be used
    if ! cli.quiet {
        let msg = if is_theme { "theme" } else { "image" };
        println!("[{info}] {img}: {f}", f = cli.file.display(), img = msg.magenta().bold());
        conf.print();
    }


    // Whether to load data from cache or to generate one from scratch
    if !cli.quiet && cli.overwrite_cache { println!("[{info}] {c}: Overwriting cache, if one present, `-c` flag provided.", c = "cache".magenta().bold()); }

    let colors = if is_theme {
        themes::built_in_theme(cli.theme.unwrap())?
    } else if !cli.overwrite_cache && cached_data.is_cached() {
        if ! cli.quiet { println!("[{info}] {c}: Using cache {}", cached_data.path.italic(), c = "cache".magenta().bold()); }
        cached_data.read()?
    } else {
        // generate colors
        if ! cli.quiet {
            let mut sp = Spinner::with_timer(Spinners::Pong, "Generating color scheme..".into());
            let not_enough = format!("[{}] Not enough colors in the image, artificially generating new colors..\n", "W".red().bold());

            //ugly workaround for printing warning, gotta stop the spinner first
            match gen_colors(&cli.file, &conf) {
                Ok((o, warn)) => {
                    sp.stop_with_message(format!("{m}[{info}] Color scheme palette generated!", m = if warn { not_enough } else { "".into() }));
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
        colors.sequences(&cache_path)?;
    }

    // write entries `[[entry]]` of the config file (if any)
    if let Some(s) = conf.entry {
        if ! cli.quiet { println!("[{info}] {}: Writing templates..", "templates".magenta().bold()); }
        template::write_template(&config_path, &cli.file, &s, &colors, cli.quiet)?
    }

    // Cache colors
    if !cli.quiet && cli.no_cache { println!("[{info}] {}: Skipping caching the palette, `-n` flag provided.", "cache".magenta().bold()); }
    if !cli.no_cache && !cached_data.is_cached() {
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
