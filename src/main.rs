//use std::collections::HashMap;
//use std::io::Cursor;
//use colorsys::{ColorAlpha, Hsl, Rgb};

use clap::Parser;
use anyhow::Result;

mod args;
mod config;
mod colors;
mod backends;
mod delta;
use args::Cli;
use colors::*;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let conf = config::parse_conf()?;
    // parse the image
    let colors = match conf.parser {
        config::Parser::Full => backends::full(&cli.file)?,
        config::Parser::Resized => backends::resized(&cli.file)?,
    };

    match conf.entry {
        None => (),
        Some(s) => config::write_template(s, &colors)?,
    };

    colors.print();

    Ok(())
}
