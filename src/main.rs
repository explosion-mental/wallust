//! wallust - Generate a colorscheme based on an image
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;

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

    // print some info that's gonna be used
    if ! cli.quiet {
        println!(" > image: {img}\n > {back} backend parser\n > Threshold of {th}\n > Using {filter} filter\n > {cs} color space",
            back = conf.backend.bold().color(conf.backend.col()),
            th = conf.threshold.bold().color(conf.threshold_col()),
            filter = conf.filter.bold().color(conf.filter.col()),
            cs = conf.color_space.bold().color(conf.color_space.col()),
            img = cli.file.display(),
        );
        println!(" > Generating color scheme...");
    }

    // generate hash cache file name and cache dir
    let cached_data = cache::Cache::new(cli.file.to_owned(), &conf)?;

    // Whether to load data from cache or to generate one from scratch
    let colors = if cached_data.is_cached() {
        if ! cli.quiet { println!(" > Using cache {}", cached_data.path.italic()); }
        cached_data.read()?
    } else {
        gen_colors(&cli.file, &conf)?
    };

    if ! cli.quiet {
        //TODO add print_long to list `value: color` like
        colors.print();
    }

    // Set sequences
    if ! cli.skip_sequences {
        if ! cli.quiet { println!(" > sequences: Setting terminal colors.."); }
        colors.sequences()?;
    }

    // write entries `[[entry]]` of the config file (if any)
    if let Some(s) = conf.entry {
        if ! cli.quiet { println!(" > Writing templates.."); }
        config::write_template(&s, &colors, cli.quiet)?
    }

    // Cache colors
    if ! cached_data.is_cached() {
        if ! cli.quiet { println!(" > Saving scheme to cache.."); }
        cached_data.write(&colors)?;
    }

    if ! cli.quiet { colors.done(); }

    Ok(())
}

/// How [`Colors`] is filled
fn gen_colors(file: &PathBuf, c: &config::Config) -> Result<colors::Colors> {

    let sort_ord = match c.filter {
        filters::Filters::Dark  | filters::Filters::Dark16 => colorspaces::ColorOrder::LightFirst,
        filters::Filters::Light | filters::Filters::Light16 => colorspaces::ColorOrder::DarkFirst,
    };


    // read image
    let rgbas = backends::main(file, &c.backend)?;

    // get the top 16 most used colors, ordered from the darkest to lightest. Different color
    // spaces can be used here.
    let top =  match c.color_space {
        colorspaces::ColorSpaces::Lab => colorspaces::lab::lab(&rgbas, c.threshold, false, sort_ord),
        colorspaces::ColorSpaces::LabMixed => colorspaces::lab::lab(&rgbas, c.threshold, true, sort_ord),
    };

    // Apply a [`Filters`] that returns the [`Colors`] struct
    let colors = match c.filter {
        filters::Filters::Dark => filters::dark::dark(top),
        filters::Filters::Dark16 => filters::dark16::dark16(top),
        filters::Filters::Light => filters::light::light(top),
        filters::Filters::Light16 => filters::light16::light16(top),
    };

    Ok(colors)
}

