//! wallust - Generate a colorscheme based on an image
use std::path::Path;

use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;
use spinners::{Spinner, Spinners};

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
    let update_current  = cli.globals.update_current;
    let skip_sequences  = &cli.globals.skip_sequences;
    let ignore_sequence = &cli.globals.ignore_sequence;
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
                    if ! skip_sequences && ! update_current {
                        if ! quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
                        colors.sequences(&cache_path, ignore_sequence.as_deref())?;
                    }

                    if update_current {
                        if ! quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
                        print!("{}", colors.to_seq(ignore_sequence.as_deref()));
                    }

                    if ! skip_templates {
                        conf.write_entry(&WalStr::Theme(theme.to_owned()), &colors, quiet)?;
                    }
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
            if ! skip_sequences && ! update_current {
                if ! quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
                colors.sequences(&cache_path, ignore_sequence.as_deref())?;
            }

            if update_current {
                if ! quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
                print!("{}", colors.to_seq(ignore_sequence.as_deref()));
            }

            //empty image_path cuz it's not used
            if ! skip_templates {
                conf.write_entry(&WalStr::Theme(theme), &colors, quiet)?;
            }
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
            if ! skip_sequences && ! update_current {
                if ! quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
                colors.sequences(&cache_path, ignore_sequence.as_deref())?;
            }

            if update_current {
                if ! quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
                print!("{}", colors.to_seq(ignore_sequence.as_deref()));
            }

            //empty image_path cuz it's not used
            if ! skip_templates {
                conf.write_entry(&walstr, &colors, quiet)?;
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
        args::Subcmds::Migrate => {
            use toml_edit::{DocumentMut, value};

            let dir  = conf.dir;
            let file = conf.file;
            let old  = dir.join("wallust-old.toml");

            if ! file.exists() {
                println!("Configuration file not found.");
                return Ok(());
            }

            let contents = std::fs::read_to_string(&file)?;
            let mut doc = contents.parse::<DocumentMut>()?;

            // true means quit
            let entry_is_empty;
            let template_is_empty;

            match doc.get("entry") {
                Some(entries) => {
                    let entries = match entries.as_array_of_tables() {
                        Some(s) => s,
                        None => {
                            eprintln!("Error, entry is not an array of tables.");
                            return Ok(());
                        },
                    };
                    entry_is_empty = false;
                    for (i, e) in entries.clone().into_iter().enumerate() {
                        let name = &format!("migrated{}", i + 1);
                        doc["templates"][name]["src"] = e["template"].clone();
                        doc["templates"][name]["dst"] = e["target"].clone();
                        //XXX since alias are recommended, use them.
                        //doc["templates"][name]["template"] = value(&e.template);
                        //doc["templates"][name]["target"]   = value(&e.target);
                        //doc["templates"][name]["pywal"] = e["new_engine"].as_value().unwrap_or(toml_edit));
                        match e.get("new_engine") {
                            Some(s) => doc["templates"][name]["pywal"] = s.clone(),
                            None    => doc["templates"][name]["pywal"] = value(true),
                        }
                    }
                },
                None => entry_is_empty = true,
            }

            match doc.get_mut("templates") {
                Some(templates) => {

                    template_is_empty = false;

                    let fields = match templates.as_table_mut() {
                        Some(s) => s,
                        None => {
                            eprintln!("Error, `[templates]` is wrongly formatted, please refer to the man page.");
                            return Ok(());
                        },
                    };

                    //we don't care about the string key
                    for (_, v) in fields.iter_mut() {
                        match v.get("new_engine")  {
                            Some(s) => {
                                v["pywal"] = value(!s.as_bool().expect("new_engine SHOULD be a boolean"));
                                v["new_engine"] = toml_edit::Item::None;
                            },
                            None => v["pywal"] = value(true),
                        }
                    }

                    // inline is shorter :3 (refactor all added templates as inline)
                    if let Some(t) = templates.as_inline_table_mut() { t.fmt() }
                },
                None => template_is_empty = true,
            }

            let filter_is_empty = doc.get("filter").is_none();

            if (entry_is_empty || filter_is_empty) || template_is_empty {
                println!("Config format Ok.\nIf you wish to define templates read `man wallust.5` for the config spec.");
                return Ok(());
            }

            println!("Succesfully migrated config, old format is at {}\nFor more info read `man wallust.5`", old.display());

            // hacky stuff: remove entry by being an empty array and rename palette by replace method
            doc.remove("entry");
            let new = doc.to_string();
            let new = if !filter_is_empty { new.replace("filter", "palette") } else { new };

            // renaeme the original config
            std::fs::rename(&file, &old)?;
            std::fs::write(&file, new)?;
        },
    }
    Ok(())

}

/// Usual `wallust image.png` call, without any subcommands.
// This used to be old main()
fn run(conf: &mut config::Config, cache_path: &Path, cli: &args::WallustArgs, g: &args::Globals) -> Result<()> {
    let info = "I".blue();
    let info = info.bold();

    // apply --backend or --filter or --colorspace
    conf.customs_cli(cli);

    // auto threshold
    conf.true_th = conf.threshold.unwrap_or_default();

    // generate hash cache file name and cache dir to either read or write to it
    let mut cached_data = cache::Cache::new(&cli.file, conf, cache_path)?;

    // print some info that's gonna be used
    if !g.quiet {
        let f = match cli.file.file_name() {
            Some(s) => s.to_string_lossy(),
            None => cli.file.to_string_lossy(),
        };
        println!("[{info}] {img}: {f}", img = "image".magenta().bold());
        conf.print();
    }

    // Whether to load data from cache or to generate one from scratch
    if !g.quiet && cli.overwrite_cache { println!("[{info}] {c}: Overwriting cache, if present, `-w` flag provided.", c = "cache".magenta().bold()); }

    let colors = if !cli.overwrite_cache && cached_data.is_cached() {
        if !g.quiet { println!("[{info}] {c}: Using cache {}", cached_data.italic(), c = "cache".magenta().bold()); }
        cached_data.read()?
    } else {
        // generate colors
        if !g.quiet {
            let mut sp = Spinner::with_timer(Spinners::Pong, "Generating color scheme..".into());

            //ugly workaround for printing warning, gotta stop the spinner first
            match gen_colors(&cli.file, conf, cli.dynamic_threshold) {
                Ok((o, warn)) => {
                    let gen = conf.fallback_generator.unwrap_or_default();
                    let not_enough = format!(
                    "[{info}] Not enough colors in the image, artificially generating new colors...\n[{info}] {method}: Using {g} to fill the palette\n",
                        g = gen.to_string().color(gen.col()),
                        method = "fallback generation method".magenta().bold()
                        );
                    sp.stop_with_message(format!("{m}[{info}] Color scheme palette generated!", m = if warn { not_enough } else { "".into() }));
                    cached_data.reached_gen();
                    o
                }
                Err(e) => {
                    sp.stop_with_message("".into());
                    return Err(e);
                },
            }
        } else {
            gen_colors(&cli.file, conf, cli.dynamic_threshold)?.0
        }
    };

    if !g.quiet {
        //TODO add print_long to list `value: color` like
        colors.print();
    }

    // Set sequences
    if !g.skip_sequences && !g.update_current {
        if !g.quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
        colors.sequences(cache_path, g.ignore_sequence.as_deref())?;
    }

    if g.update_current {
        if !g.quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
        print!("{}", colors.to_seq(g.ignore_sequence.as_deref()));
    }

    if !g.skip_templates {
        conf.write_entry(&WalStr::Path(cli.file.clone()), &colors, g.quiet)?;
    }

    // Cache colors
    if !g.quiet && cli.no_cache { println!("[{info}] {}: Skipping caching the palette, `-n` flag provided.", "cache".magenta().bold()); }
    if !cli.no_cache && !cached_data.is_cached() {
        if !g.quiet { println!("[{info}] {}: Saving scheme to cache.", "cache".magenta().bold()); }
        cached_data.write(&colors)?;
    }

    if !g.quiet { colors.done(); }

    Ok(())
}
