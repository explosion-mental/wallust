//use std::collections::HashMap;
//use std::io::Cursor;
//use colorsys::{ColorAlpha, Hsl, Rgb};
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;
use owo_colors::{OwoColorize, AnsiColors};

mod args;
mod config;
mod colors;
mod backends;
mod delta;
use args::Cli;
use colors::*;
use config::Config;

//TODO handle errors
//XXX BTree?
//XXX generate an actual scheme, rather than listing colors¿

/// #About LAB
/// > The lightness value, L*, also referred to as "Lstar," defines black at 0 and white at 100.
/// > The a* axis is relative to the green-red opponent colors, with negative values toward green
/// > and positive > values toward red.
/// > The b* axis represents the blue-yellow opponents, with negative numbers toward
/// > blue and positive toward yellow.
/// ref: <https://en.wikipedia.org/wiki/CIELAB_color_space>

impl Config {
    pub fn parse(&self, file: &PathBuf) -> Result<Colors<MyLab>> {
        match self.parser {
            config::Parser::Full => backends::full(file, self.threshold),
            config::Parser::Resized => backends::resized(file, self.threshold),
        }
    }
    pub fn print(&self) {
        let col = match self.parser {
            config::Parser::Full => AnsiColors::Blue,
            config::Parser::Resized => AnsiColors::Cyan,
        };
        println!("Using {} backend parser", self.parser.bold().color(col));
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let conf = config::parse_conf()?;
    println!("Generating color scheme...");
    conf.print();
    // parse the image
    let colors = conf.parse(&cli.file)?;

    let entries = &conf.entry;

    match entries {
        Some(s) => config::write_template(s, &colors)?,
        None => (),
    };

    colors.print();

    Ok(())
}
