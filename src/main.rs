//! wallust - Generate a colorscheme based on an image
use clap::Parser;
use anyhow::Result;
use owo_colors::{OwoColorize, AnsiColors};

mod args;
mod config;
mod colors;
mod backends;
mod cache;
use colors::*;
use config::Config;

//TODO handle errors
//XXX BTree?
//XXX generate an actual scheme, rather than listing colors¿

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let conf = Config::new()?;

    if ! cli.quiet {
        println!("Generating color scheme...");
        println!("Using {} backend parser with a threshold of {}",
            conf.backend.bold().color(conf.backend_col()),
            conf.threshold.bold().color(conf.threshold_col()),
        );
    }

    //workaround around ref and lifetimes
    let p = cli.file.to_owned();
    let bend = conf.backend;

    // Whether to load data from cache or to generate from scratch
    let cached_data = cache::Cache::new(p, bend, conf.threshold)?;
    let colors = if cached_data.is_cached() { cached_data.read()? } else { backends::gen_colors(&cli.file, &conf.backend, conf.threshold)? };

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
    pub fn backend_col(&self) -> AnsiColors {
        match self.backend {
            backends::Backend::Full => AnsiColors::Blue,
            backends::Backend::Resized => AnsiColors::Cyan,
        }
    }
}
