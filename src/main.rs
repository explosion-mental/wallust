//! wallust - Generate a colorscheme based on an image
use std::path::Path;

use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;

use wallust::{
    args, cache, config::{self, WalStr}, gen_colors, themes
};

const ISSUE: &str = "please report this at <https://codeberg.org/explosion-mental/wallust/issues>";

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let info = "I".blue();
    let info = info.bold();

    // init directories
    let Some(cache_path) = dirs::cache_dir() else {
        anyhow::bail!("The cache path for the platform could not be found, {ISSUE}");
    };

    // globals
    let quiet = cli.globals.quiet;
    let skip_templates  = &cli.globals.skip_templates;

    let mut conf = config::Config::new(&cli.globals)?;

    match cli.subcmds {
        args::Subcmds::Run(s) => {
            // use serde to read wallust.toml, this is mut only because the user could provide a `-C custom_config.toml`
            run(&mut conf, &cache_path, &s, &cli.globals)?
        },
        args::Subcmds::Pywal(s) => {
            match s.file {
                Some(_) => run(&mut conf, &cache_path, &s.into(), &cli.globals)?, // -i "...png"
                None => { //must be using a file or a theme name `-f file.jpg`
                    let theme = &s.theme.expect("SHOULD BE NON EMPTY, from clap");
                    if !quiet { println!("[{info}] {}: Using {theme}", "theme".magenta().bold(), theme = theme.italic()); }
                    let colors = themes::built_in_theme(theme, quiet).ok_or_else(||anyhow::anyhow!("Theme not found. Quitting..."))?;
                    colors.print();

                    cli.globals.set_seq(&colors, &cache_path)?;
                    cli.globals.update_cur(&colors)?;
                    if ! skip_templates { conf.write_entry(&WalStr::Theme(theme.to_owned()), &colors, quiet)?; }
                }
            }
        },
        #[cfg(feature = "themes")]
        args::Subcmds::Theme { theme, preview } => {
            if theme == themes::LIST { // wallust theme list
                if !quiet { themes::list_themes(); }
                return Ok(())
            }

            if !quiet && !preview { println!("[{info}] {}: Using {theme}", "theme".magenta().bold(), theme = theme.italic()); }
            let colors = themes::built_in_theme(&theme, quiet).ok_or_else(||anyhow::anyhow!("Theme not found. Quitting..."))?;
            if ! quiet {
                    colors.print();
                    if preview { return Ok(()); } //exit if preview
            }

            cli.globals.set_seq(&colors, &cache_path)?;
            cli.globals.update_cur(&colors)?;

            //empty image_path cuz it's not used
            if ! skip_templates { conf.write_entry(&WalStr::Theme(theme), &colors, quiet)?; }
            if ! quiet { colors.done() }
        },
        args::Subcmds::Cs { colorscheme, format } => {
            let (walstr, colors) = themes::search_theme_or_cs(&colorscheme, quiet, &conf.dir, format)?;

            let msg = match walstr {
                WalStr::Path(ref p) => format!("Using a colorscheme from file {}", p.display()),
                WalStr::Theme(ref p) => format!("Using the theme {p}"),
            };


            if ! quiet { println!("[{info}] {cs}: {msg}", cs = "colorscheme".magenta().bold()); }
            if ! quiet { colors.print(); }

            cli.globals.set_seq(&colors, &cache_path)?;
            cli.globals.update_cur(&colors)?;

            //empty image_path cuz it's not used
            if ! skip_templates { conf.write_entry(&walstr, &colors, quiet)?;
            }
            if ! quiet { colors.done() }
        },
        args::Subcmds::Debug => {
            use cache::CACHE_VER;
            println!(
"Cache version: {CACHE_VER}
Cache path: {}
{conf}
 ~ make sure to report any issue at <https://codeberg.org/explosion-mental/wallust/issues> ~ ",
        cache_path.display(),
            );
        },
        args::Subcmds::Migrate => wallust::args::migrate(&conf)?,
    }

    Ok(())
}

/// Usual `wallust image.png` call, without any subcommands.
fn run(conf: &mut config::Config, cache_path: &Path, cli: &args::WallustArgs, g: &args::Globals) -> Result<()> {
    let info = "I".blue();
    let info = info.bold();

    // When to be quiet. --print-scheme requires no other output.
    let quiet = g.quiet || cli.print_scheme;

    // apply --backend or --filter or --colorspace
    conf.customs_cli(cli);

    // auto threshold
    conf.true_th = conf.threshold.unwrap_or_default();

    // generate hash cache file name and cache dir to either read or write to it
    // let mut cached_data = cache::Cache::new(&cli.file, conf, cache_path)?;

    // print some info that's gonna be used
    if !quiet {
        let f = match cli.file.file_name() {
            Some(s) => s.to_string_lossy(),
            None => cli.file.to_string_lossy(),
        };
        println!("[{info}] {img}: {f}", img = "image".magenta().bold());
        conf.print();
    }

    // Whether to load data from cache or to generate one from scratch
    if !quiet && cli.overwrite_cache { println!("[{info}] {c}: Overwriting cache, if present, `-w` flag provided.", c = "cache".magenta().bold()); }

    let colors = gen_colors(&cli.file, conf, cli.dynamic_threshold, cache_path, cli.no_cache, quiet, cli.overwrite_cache)?;

    if !quiet { colors.print(); }
    g.set_seq(&colors, cache_path)?;
    g.update_cur(&colors)?;
    if !g.skip_templates { conf.write_entry(&WalStr::Path(cli.file.clone()), &colors, quiet)?; }
    if !g.no_hooks { conf.run_hooks(quiet); }

    // Cache colors
    if !quiet && cli.no_cache { println!("[{info}] {}: Skipping caching the palette, `-n` flag provided.", "cache".magenta().bold()); }
    if !cli.no_cache && !quiet { println!("[{info}] {}: Saving scheme to cache.", "cache".magenta().bold()); }
    if cli.save_scheme {
        let f = match cli.file.file_prefix() {
            Some(s) => s.to_string_lossy(),
            None => cli.file.to_string_lossy(),
        };
        let p = conf.dir.join("colorschemes").join(format!("{f}.json"));
        cache::write_json(p, &colors, &conf.dir.display().to_string(), true)?;
        if !quiet { println!("[{info}] {} into colorscheme directory.", "Saving Scheme".bold()); }
    }

    if cli.print_scheme { colors.output_nl(); }

    if !quiet { colors.done(); }
    Ok(())
}
