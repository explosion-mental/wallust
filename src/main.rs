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

    if ! cli.quiet {
        println!("Generating color scheme...");
        println!("- {} backend parser\n- threshold of {}\n- {} filter\n- {} color space",
            conf.backend.bold().color(conf.backend.col()),
            conf.threshold.bold().color(conf.threshold_col()),
            conf.filter.bold().color(conf.filter.col()),
            conf.color_space.bold().color(conf.color_space.col()),
        );
    }

    // Whether to load data from cache or to generate from scratch
    let cached_data = cache::Cache::new(cli.file.to_owned(), &conf)?;
    let colors = if cached_data.is_cached() {
        if ! cli.quiet { println!("- Using cache {}", cached_data.path.italic()); }
        cached_data.read()?
    } else {
        gen_colors(&cli.file, &conf)?
    };

    // Cache colors
    if ! cached_data.is_cached() { cached_data.write(&colors)?; }

    // write entries `[[entry]]` of the config file (if any)
    if let Some(s) = conf.entry { config::write_template(&s, &colors)? }

    if ! cli.quiet {
        //TODO add print_long to list `value: color` like
        colors.print();
    }

    Ok(())
}

/// How [`Colors`] is filled
fn gen_colors(file: &PathBuf, c: &config::Config) -> Result<colors::Colors> {
    // read image
    let rgbas = backends::main(file, &c.backend)?;

    // get the top 16 most used colors, ordered from the darkest to lightest. Different color
    // spaces can be used here.
    let top = colorspaces::main(&rgbas, c.threshold, &c.color_space);

    // Apply a [`Filters`] that returns the [`Colors`] struct
    let colors = filters::main(top, &c.filter);

    Ok(colors)
}

