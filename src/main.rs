//! wallust - Generate a colorscheme based on an image
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;
use owo_colors::{OwoColorize, AnsiColors};

mod args;
mod config;
mod colors;
mod backends;
mod filters;
mod cache;
mod colorspaces;
use colors::{Colors, Myrgb};
use config::Config;

//TODO handle errors
//XXX BTree?
//XXX generate an actual scheme, rather than listing colors¿

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let conf = Config::new()?;

    if ! cli.quiet {
        println!("Generating color scheme...");
        println!("- {} backend parser\n- threshold of {}\n- {} filter\n- {} color space",
            conf.backend.bold().color(conf.backend.col()),
            conf.threshold.bold().color(conf.threshold_col()),
            conf.filter.bold().color(conf.filter.col()),
            conf.color_space.bold().color(conf.color_space.col()),
        );
    }

    //workaround around ref and lifetimes
    let p = cli.file.to_owned();
    let bend = conf.backend;

    // Whether to load data from cache or to generate from scratch
    let cached_data = cache::Cache::new(p, bend, conf.threshold)?;
    let colors = if cached_data.is_cached() { cached_data.read()? } else { gen_colors(&cli.file, &conf)? };

    // Cache colors
    cached_data.write(&colors)?;

    // write entries `[[entry]]` of the config file (if any)
    if let Some(s) = conf.entry { config::write_template(&s, &colors)? }

    if ! cli.quiet {
        //TODO add print_long to list `value: color` like
        colors.print();
    }

    Ok(())
}

/// main fn that calls other methods, used in main.rs
fn gen_colors(file: &PathBuf, c: &Config) -> Result<Colors> {
    // read image
    let rgbas = backends::main(file, &c.backend)?;

    // get the top 8 most used colors, ordered from the lightess to the darkess. Different color
    // spaces could be used here.
    let histo = colorspaces::main(&rgbas, c.threshold, &c.color_space, c.mix_colors);

    // Apply a [`Filters`] that returns the [`Colors`] struct
    let colors = filters::main(histo, &c.filter);

    Ok(colors)
}

impl Config {
    pub fn threshold_col(&self) -> AnsiColors {
        match self.threshold {
            1 => AnsiColors::Yellow,
            2 => AnsiColors::Cyan,
            3..=10 => AnsiColors::Green,
            11..=49 => AnsiColors::Blue,
            50..=100 => AnsiColors::Red,
            _ => AnsiColors::Red,
        }
    }
}
