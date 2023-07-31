//! wallust - Generate a colorscheme based on an image
use std::path::Path;

use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;
use spinners::{Spinner, Spinners};

use wallust::{
    args,
    backends,
    cache,
    colors,
    colorspaces,
    config,
    filters,
    themes,
};

const ISSUE: &str = "please report this at <https://codeberg.org/explosion-mental/wallust/issues>";

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let info = "I".blue();
    let info = info.bold();

    // init directories
    let Some(original_config_path) = dirs::config_dir() else {
        anyhow::bail!("Config path for the platform could not be found, {ISSUE}");
    };
    let Some(cache_path) = dirs::cache_dir() else {
        anyhow::bail!("The cache path for the platform could not be found, {ISSUE}");
    };

    // check config file or generate one if not one isn't found
    let config_cli = match &cli.args {
        Some(s) => s.config_path.as_ref(),
        None => None,
    };

    let mut is_orig_conf = true;

    // check config dir
    let config_path =
        match &cli.args {
            Some(s) => {
                match &s.config_dir {
                    Some(path) => { //only in this case, the config dir is altered
                        is_orig_conf = false;
                        path
                    },
                    None => &original_config_path,
                }
            },
            None => &original_config_path,
        };

    // this is mut only because the user could provide a `-C custom_config.toml`
    let mut conf = config::Config::new(&config_path, config_cli, is_orig_conf)?;

    match &cli.args {
        Some(s) => no_subcomands(&mut conf, &cache_path, &s)?,
        None => (),
    }

    match cli.subcmds {
        #[cfg(feature = "themes")]
        Some(args::Subcmds::Theme { theme, quiet, skip_sequences }) => {
            if ! quiet { println!("[{info}] {}: Using {theme}", "theme".magenta().bold(), theme = theme.italic()); }
            let colors = themes::built_in_theme(theme, quiet)?;
            if ! quiet { colors.print(); }
            if ! skip_sequences {
                if ! quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
                colors.sequences(&cache_path)?;
            }
            let path = std::path::Path::new("");

            conf.write_entry(&path, &colors, quiet)?;
            if ! quiet { colors.done() }
        },
        Some(args::Subcmds::Cs { file, quiet, skip_sequences, format }) => {
            if ! quiet { println!("[{info}] {cs}: from file {}", file.display(), cs = "colorscheme".magenta().bold()); }
            // read_scheme or try_all_schemes
            let colors = match format {
                Some(s) => themes::read_scheme(&file, s)?,
                None => themes::try_all_schemes(&file)?,
            };

            if ! quiet { colors.print(); }
            if ! skip_sequences {
                if ! quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
                colors.sequences(&cache_path)?;
            }
            let path = std::path::Path::new("");

            conf.write_entry(&path, &colors, quiet)?;
            if ! quiet { colors.done() }

        },
        None => (),
    }

    Ok(())

}

/// Usual `wallust image.png` call, without any subcommands.
// This used to be old main()
fn no_subcomands(conf: &mut config::Config, cache_path: &Path, cli: &args::WallustArgs) -> Result<()> {
    let info = "I".blue();
    let info = info.bold();

    // apply --backend or --filter or --colorspace
    conf.customs_cli(&cli);

    // generate hash cache file name and cache dir to either read or write to it
    let cached_data = cache::Cache::new(&cli.file, &conf, &cache_path)?;

    // print some info that's gonna be used
    if ! cli.quiet {
        println!("[{info}] {img}: {f}", f = cli.file.display(), img = "image".magenta().bold());
        conf.print();
    }

    // Whether to load data from cache or to generate one from scratch
    if !cli.quiet && cli.overwrite_cache { println!("[{info}] {c}: Overwriting cache, if one present, `-c` flag provided.", c = "cache".magenta().bold()); }

    let colors = if !cli.overwrite_cache && cached_data.is_cached() {
        if ! cli.quiet { println!("[{info}] {c}: Using cache {}", cached_data.path.italic(), c = "cache".magenta().bold()); }
        cached_data.read()?
    } else {
        // generate colors
        if ! cli.quiet {
            let mut sp = Spinner::with_timer(Spinners::Pong, "Generating color scheme..".into());

            //ugly workaround for printing warning, gotta stop the spinner first
            match gen_colors(&cli.file, &conf) {
                Ok((o, warn)) => {
                    let not_enough = format!("[{}] Not enough colors in the image, artificially generating new colors..\n", "W".red().bold());
                    sp.stop_with_message(format!("{m}[{info}] Color scheme palette generated!", m = if warn { not_enough } else { "".into() }));
                    o
                }
                Err(e) => {
                    sp.stop_with_message("".into());
                    return Err(e);
                },
            }
        } else {
            gen_colors(&cli.file, &conf)?.0
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

    conf.write_entry(&cli.file, &colors, cli.quiet)?;

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
fn gen_colors(file: &Path, c: &config::Config) -> Result<(colors::Colors, bool)> {
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
