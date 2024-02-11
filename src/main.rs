//! wallust - Generate a colorscheme based on an image
use std::path::Path;

use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;
use spinners::{Spinner, Spinners};

use wallust::{
    args,
    cache,
    config,
    config::WalStr,
    themes,
    gen_colors,
};

const ISSUE: &str = "please report this at <https://codeberg.org/explosion-mental/wallust/issues>";

fn main() -> Result<()> {
    let cli = args::Subcmds::parse();
    let info = "I".blue();
    let info = info.bold();

    // init directories
    let Some(original_config_path) = dirs::config_dir() else {
        anyhow::bail!("Config path for the platform could not be found, {ISSUE}");
    };
    let Some(cache_path) = dirs::cache_dir() else {
        anyhow::bail!("The cache path for the platform could not be found, {ISSUE}");
    };

    match cli {
        args::Subcmds::Run(s) => {
            // use serde to read wallust.toml, this is mut only because the user could provide a `-C custom_config.toml`
            let mut conf = config::Config::new(&original_config_path, s.config_path.as_deref(), s.config_dir.as_deref())?;
            run(&mut conf, &cache_path, &s)?
        },
        #[cfg(feature = "themes")]
        args::Subcmds::Theme { theme, quiet, skip_sequences, skip_templates, preview, update_current } => {
            let conf = config::Config::new(&original_config_path, None, None)?;
            if !quiet && !preview { println!("[{info}] {}: Using {theme}", "theme".magenta().bold(), theme = theme.italic()); }
            let colors = themes::built_in_theme(&theme, quiet)?;
            if ! quiet {
                    colors.print();
                    if preview { return Ok(()); } //exit if preview
            }
            if ! skip_sequences && ! update_current {
                if ! quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
                colors.sequences(&cache_path)?;
            }

            if update_current {
                if ! quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
                print!("{}", colors.to_seq());
            }

            //empty image_path cuz it's not used
            if ! skip_templates {
                conf.write_entry(&WalStr::Theme(&theme), &colors, quiet)?;
            }
            if ! quiet { colors.done() }
        },
        args::Subcmds::Cs { file, quiet, skip_sequences, skip_templates, format, update_current } => {
            let conf = config::Config::new(&original_config_path, None, None)?;
            if ! quiet { println!("[{info}] {cs}: from file {}", file.display(), cs = "colorscheme".magenta().bold()); }
            // read_scheme or try_all_schemes
            let colors = match format {
                Some(s) => themes::read_scheme(&file, &s)?,
                None => themes::try_all_schemes(&file)?,
            };

            if ! quiet { colors.print(); }
            if ! skip_sequences && ! update_current {
                if ! quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
                colors.sequences(&cache_path)?;
            }

            if update_current {
                if ! quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
                print!("{}", colors.to_seq());
            }

            //empty image_path cuz it's not used
            if ! skip_templates {
                conf.write_entry(&WalStr::Path(&file), &colors, quiet)?;
            }
            if ! quiet { colors.done() }

        },
        args::Subcmds::Debug => {
            let conf = config::Config::new(&original_config_path, None, None)?;
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
            use toml_edit::{Document, value};

            let dir  = original_config_path.join("wallust");
            let file = dir.join("wallust.toml");
            let old  = dir.join("wallust-old.toml");

            if ! file.exists() {
                println!("Configuration file not found.");
                return Ok(());
            }

            let contents = std::fs::read_to_string(&file)?;
            let mut doc = contents.parse::<Document>()?;
            let conf: config::Config = toml::from_str(&contents)?;

            // true means quit
            let entryflag;
            let filterflag;

            match &conf.entry {
                Some(entries) => {
                    entryflag = false;
                    for (i, e) in entries.iter().enumerate() {
                        let name = &format!("migrated{}", i + 1);
                        doc["templates"][name]["src"] = value(&e.template);
                        doc["templates"][name]["dst"]   = value(&e.target);
                        //XXX since alias are recommended, use them.
                        //doc["templates"][name]["template"] = value(&e.template);
                        //doc["templates"][name]["target"]   = value(&e.target);
                        match e.new_engine {
                            Some(s) => if s == false { doc["templates"][name]["pywal"] = value(true) },
                            None => doc["templates"][name]["pywal"] = value(true),
                        }
                    }
                },
                None => entryflag = true,
            }

            match doc["filter"].as_value() {
                Some(_) => filterflag = false,
                None    => filterflag = true,
            }

            if entryflag && filterflag {
                println!("No templates are used, quitting.\nIf you wish to define templates read `man wallust.5` for the config spec.");
                return Ok(());
            }

            // inline is shorter :3
            doc["templates"].as_inline_table_mut().map(|t| t.fmt());

            println!("Succesfully migrated config, old format is at {}\nFor more info read `man wallust.5`", old.display());

            // hacky stuff: remove entry by being an empty array and rename palette by replace method
            doc["entry"] = toml_edit::array();
            let new = doc.to_string();
            let new = if filterflag { new.replace("filter", "palette") } else { new };

            // renaeme the original config
            std::fs::rename(&file, &old)?;
            std::fs::write(&file, &new)?;
        }
    }
    Ok(())

}

/// Usual `wallust image.png` call, without any subcommands.
// This used to be old main()
fn run(conf: &mut config::Config, cache_path: &Path, cli: &args::WallustArgs) -> Result<()> {
    let info = "I".blue();
    let info = info.bold();

    // apply --backend or --filter or --colorspace
    conf.customs_cli(cli);

    // generate hash cache file name and cache dir to either read or write to it
    let mut cached_data = cache::Cache::new(&cli.file, conf, cache_path)?;

    // print some info that's gonna be used
    if ! cli.quiet {
        println!("[{info}] {img}: {f}", f = cli.file.display(), img = "image".magenta().bold());
        conf.print();
    }

    // Whether to load data from cache or to generate one from scratch
    if !cli.quiet && cli.overwrite_cache { println!("[{info}] {c}: Overwriting cache, if present, `-w` flag provided.", c = "cache".magenta().bold()); }

    let colors = if !cli.overwrite_cache && cached_data.is_cached() {
        if ! cli.quiet { println!("[{info}] {c}: Using cache {}", cached_data.italic(), c = "cache".magenta().bold()); }
        cached_data.read()?
    } else {
        // generate colors
        if ! cli.quiet {
            let mut sp = Spinner::with_timer(Spinners::Pong, "Generating color scheme..".into());

            //ugly workaround for printing warning, gotta stop the spinner first
            match gen_colors(&cli.file, conf) {
                Ok((o, warn)) => {
                    let gen = conf.generation.unwrap_or_default();
                    let not_enough = format!(
                    "[{info}] Not enough colors in the image, artificially generating new colors...\n[{info}] {method}: Using {g} to fill the palette\n",
                        g = gen.to_string().color(gen.col()),
                        method = "generation method".magenta().bold()
                        );
                    sp.stop_with_message(format!("{m}[{info}] Color scheme palette generated!", m = if warn { not_enough } else { "".into() }));
                    cached_data.gen(&gen);
                    o
                }
                Err(e) => {
                    sp.stop_with_message("".into());
                    return Err(e);
                },
            }
        } else {
            gen_colors(&cli.file, conf)?.0
        }
    };

    if ! cli.quiet {
        //TODO add print_long to list `value: color` like
        colors.print();
    }

    // Set sequences
    if ! cli.skip_sequences && ! cli.update_current {
        if ! cli.quiet { println!("[{info}] {}: Setting terminal colors.", "sequences".magenta().bold()); }
        colors.sequences(cache_path)?;
    }

    if cli.update_current {
        if ! cli.quiet { println!("[{info}] {seq}: Setting colors {b} in the current terminal.", seq = "sequences".magenta().bold(), b = "only".bold()); }
        print!("{}", colors.to_seq());
    }

    if ! cli.skip_templates {
        conf.write_entry(&WalStr::Path(&cli.file), &colors, cli.quiet)?;
    }

    // Cache colors
    if !cli.quiet && cli.no_cache { println!("[{info}] {}: Skipping caching the palette, `-n` flag provided.", "cache".magenta().bold()); }
    if !cli.no_cache && !cached_data.is_cached() {
        if ! cli.quiet { println!("[{info}] {}: Saving scheme to cache.", "cache".magenta().bold()); }
        cached_data.write(&colors)?;
    }

    if ! cli.quiet { colors.done(); }

    Ok(())
}
