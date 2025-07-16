//! wallust migrate
//! Module that handles the `migrate` subcommand
use anyhow::Result;

use crate::config::Config;

pub fn migrate(conf: &Config) -> Result<()> {
    use toml_edit::{DocumentMut, value};

    let dir  = &conf.dir;
    let file = &conf.file;
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
    Ok(())
}
