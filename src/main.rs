//! wallust - Generate a colorscheme based on an image
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;
use owo_colors::{OwoColorize, AnsiColors};

mod args;
mod config;
mod colors;
mod backends;
mod delta;
mod cache;
use args::Cli;
use colors::*;
use config::Config;

//TODO handle errors
//XXX BTree?
//XXX generate an actual scheme, rather than listing colors¿

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conf = config::parse_conf()?;

    println!("Generating color scheme...");
    println!("Using {} backend parser with a threshold of {}",
        conf.parser.bold().color(conf.backend_col()),
        conf.threshold.bold().color(conf.threshold_col()),
    );

    //workaround around ref and lifetimes
    let p = cli.file.to_owned();
    let bend = conf.parser;

    // Whether to load data from cache or to generate from scratch
    let cached_data = cache::Cache::new(p, bend, conf.threshold)?;
    let colors = if cached_data.is_cached() { cached_data.read()? } else { conf.gen_colors(&cli.file)? };

    // Cache colors
    cached_data.write(&colors)?;

    // match entries `[[entry]]` of the config file (if any)
    match conf.entry {
        Some(s) => config::write_template(&s, &colors)?,
        None => (),
    };

    //TODO add print_long to list `value: color` like
    colors.print();

    Ok(())
}

impl Config {
    pub fn gen_colors(&self, file: &PathBuf) -> Result<Colors<MyLab>> {
        match self.parser {
            config::Backend::Full => backends::full(file, self.threshold),
            config::Backend::Resized => backends::resized(file, self.threshold),
        }
    }
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
        match self.parser {
            config::Backend::Full => AnsiColors::Blue,
            config::Backend::Resized => AnsiColors::Cyan,
        }
    }
}
