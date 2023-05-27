use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use crate::config::Entries;
use crate::colors::Colors;

use anyhow::{Result, Context};
use new_string_template::template::Template;
use owo_colors::OwoColorize;

/// Writes `template`s into `target`s
pub fn write_template(entries: &[Entries], values: &Colors, quiet: bool) -> Result<()>{
    let Some(config) = dirs::config_dir() else {
        anyhow::bail!(
            "Config path for the platform wasn't found,
please report this at <https://codeberg.org/explosion-mental/wallust/issues>");
    };
    let config = config.display().to_string() + "/wallust/";

    // contents of config files
    let mut contents = vec![];

    // gather `String`s of the contents of the entries (in order to cast it down to &str)
    for e in entries {
        let path = config.to_owned() + &e.template;
        let file_template = match read_to_string(&path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[{w}] Skipping {path}: {e}", w = "W".red().bold());
                continue;
            }
        };
        contents.push( (&e.target, file_template) );
    }

    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path
    for (target, file_content) in &contents {
        let rendered = Template::new(file_content).render(&values.to_hash())
            .with_context(|| format!("Templating failed with {}:", target))?;

        //XXX on `shellexpand`, think about using `::full()` to support env vars. Seems a bit sketchy/sus
        let mut buffer = File::create(shellexpand::tilde(target).as_ref())
            .with_context(|| format!("Failed to create file {}:", target))?;

        buffer.write_all(rendered.as_bytes())
            .with_context(|| format!("Failed to write to file {}:", target))?;
        if ! quiet { println!("    * {} ... OK", target); }
    }
    Ok(())
}

impl Colors {
    pub fn to_hash(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        //XXX instead of multiple `.method()` maybe using enums and match with a single method

        // normal output `#EEEEEE`
        map.insert("color0" , self.color0 .to_string());
        map.insert("color1" , self.color1 .to_string());
        map.insert("color2" , self.color2 .to_string());
        map.insert("color3" , self.color3 .to_string());
        map.insert("color4" , self.color4 .to_string());
        map.insert("color5" , self.color5 .to_string());
        map.insert("color6" , self.color6 .to_string());
        map.insert("color7" , self.color7 .to_string());
        map.insert("color8" , self.color8 .to_string());
        map.insert("color9" , self.color9 .to_string());
        map.insert("color10", self.color10.to_string());
        map.insert("color11", self.color11.to_string());
        map.insert("color12", self.color12.to_string());
        map.insert("color13", self.color13.to_string());
        map.insert("color14", self.color14.to_string());
        map.insert("color15", self.color15.to_string());
        map.insert("foreground", self.foreground.to_string());
        map.insert("background", self.background.to_string());

        //.rgb output `235,235,235`
        map.insert("color0.rgb" , self.color0 .rgb());
        map.insert("color1.rgb" , self.color1 .rgb());
        map.insert("color2.rgb" , self.color2 .rgb());
        map.insert("color3.rgb" , self.color3 .rgb());
        map.insert("color4.rgb" , self.color4 .rgb());
        map.insert("color5.rgb" , self.color5 .rgb());
        map.insert("color6.rgb" , self.color6 .rgb());
        map.insert("color7.rgb" , self.color7 .rgb());
        map.insert("color8.rgb" , self.color8 .rgb());
        map.insert("color9.rgb" , self.color9 .rgb());
        map.insert("color10.rgb", self.color10.rgb());
        map.insert("color11.rgb", self.color11.rgb());
        map.insert("color12.rgb", self.color12.rgb());
        map.insert("color13.rgb", self.color13.rgb());
        map.insert("color14.rgb", self.color14.rgb());
        map.insert("color15.rgb", self.color15.rgb());
        map.insert("foreground.rgb", self.foreground.rgb());
        map.insert("background.rgb", self.background.rgb());

        //.rgba output `235,235,235,1.0`
        map.insert("color0.rgba" , self.color0 .rgba());
        map.insert("color1.rgba" , self.color1 .rgba());
        map.insert("color2.rgba" , self.color2 .rgba());
        map.insert("color3.rgba" , self.color3 .rgba());
        map.insert("color4.rgba" , self.color4 .rgba());
        map.insert("color5.rgba" , self.color5 .rgba());
        map.insert("color6.rgba" , self.color6 .rgba());
        map.insert("color7.rgba" , self.color7 .rgba());
        map.insert("color8.rgba" , self.color8 .rgba());
        map.insert("color9.rgba" , self.color9 .rgba());
        map.insert("color10.rgba", self.color10.rgba());
        map.insert("color11.rgba", self.color11.rgba());
        map.insert("color12.rgba", self.color12.rgba());
        map.insert("color13.rgba", self.color13.rgba());
        map.insert("color14.rgba", self.color14.rgba());
        map.insert("color15.rgba", self.color15.rgba());
        map.insert("foreground.rgba", self.foreground.rgba());
        map.insert("background.rgba", self.background.rgba());

        //.xrgba output `ee/ee/ee/ff`
        map.insert("color0.xrgba" , self.color0 .xrgba());
        map.insert("color1.xrgba" , self.color1 .xrgba());
        map.insert("color2.xrgba" , self.color2 .xrgba());
        map.insert("color3.xrgba" , self.color3 .xrgba());
        map.insert("color4.xrgba" , self.color4 .xrgba());
        map.insert("color5.xrgba" , self.color5 .xrgba());
        map.insert("color6.xrgba" , self.color6 .xrgba());
        map.insert("color7.xrgba" , self.color7 .xrgba());
        map.insert("color8.xrgba" , self.color8 .xrgba());
        map.insert("color9.xrgba" , self.color9 .xrgba());
        map.insert("color10.xrgba", self.color10.xrgba());
        map.insert("color11.xrgba", self.color11.xrgba());
        map.insert("color12.xrgba", self.color12.xrgba());
        map.insert("color13.xrgba", self.color13.xrgba());
        map.insert("color14.xrgba", self.color14.xrgba());
        map.insert("color15.xrgba", self.color15.xrgba());
        map.insert("foreground.xrgba", self.foreground.xrgba());
        map.insert("background.xrgba", self.background.xrgba());

        //.strip output `EEEEEE`
        map.insert("color0.strip" , self.color0 .strip());
        map.insert("color1.strip" , self.color1 .strip());
        map.insert("color2.strip" , self.color2 .strip());
        map.insert("color3.strip" , self.color3 .strip());
        map.insert("color4.strip" , self.color4 .strip());
        map.insert("color5.strip" , self.color5 .strip());
        map.insert("color6.strip" , self.color6 .strip());
        map.insert("color7.strip" , self.color7 .strip());
        map.insert("color8.strip" , self.color8 .strip());
        map.insert("color9.strip" , self.color9 .strip());
        map.insert("color10.strip", self.color10.strip());
        map.insert("color11.strip", self.color11.strip());
        map.insert("color12.strip", self.color12.strip());
        map.insert("color13.strip", self.color13.strip());
        map.insert("color14.strip", self.color14.strip());
        map.insert("color15.strip", self.color15.strip());
        map.insert("foreground.strip", self.foreground.strip());
        map.insert("background.strip", self.background.strip());

        //.red output `235`
        map.insert("color0.red" , self.color0 .red());
        map.insert("color1.red" , self.color1 .red());
        map.insert("color2.red" , self.color2 .red());
        map.insert("color3.red" , self.color3 .red());
        map.insert("color4.red" , self.color4 .red());
        map.insert("color5.red" , self.color5 .red());
        map.insert("color6.red" , self.color6 .red());
        map.insert("color7.red" , self.color7 .red());
        map.insert("color8.red" , self.color8 .red());
        map.insert("color9.red" , self.color9 .red());
        map.insert("color10.red", self.color10.red());
        map.insert("color11.red", self.color11.red());
        map.insert("color12.red", self.color12.red());
        map.insert("color13.red", self.color13.red());
        map.insert("color14.red", self.color14.red());
        map.insert("color15.red", self.color15.red());
        map.insert("foreground.red", self.foreground.red());
        map.insert("background.red", self.background.red());

        //.green output `235`
        map.insert("color0.green" , self.color0 .green());
        map.insert("color1.green" , self.color1 .green());
        map.insert("color2.green" , self.color2 .green());
        map.insert("color3.green" , self.color3 .green());
        map.insert("color4.green" , self.color4 .green());
        map.insert("color5.green" , self.color5 .green());
        map.insert("color6.green" , self.color6 .green());
        map.insert("color7.green" , self.color7 .green());
        map.insert("color8.green" , self.color8 .green());
        map.insert("color9.green" , self.color9 .green());
        map.insert("color10.green", self.color10.green());
        map.insert("color11.green", self.color11.green());
        map.insert("color12.green", self.color12.green());
        map.insert("color13.green", self.color13.green());
        map.insert("color14.green", self.color14.green());
        map.insert("color15.green", self.color15.green());
        map.insert("foreground.green", self.foreground.green());
        map.insert("background.green", self.background.green());

        //.blue output `235`
        map.insert("color0.blue" , self.color0 .blue());
        map.insert("color1.blue" , self.color1 .blue());
        map.insert("color2.blue" , self.color2 .blue());
        map.insert("color3.blue" , self.color3 .blue());
        map.insert("color4.blue" , self.color4 .blue());
        map.insert("color5.blue" , self.color5 .blue());
        map.insert("color6.blue" , self.color6 .blue());
        map.insert("color7.blue" , self.color7 .blue());
        map.insert("color8.blue" , self.color8 .blue());
        map.insert("color9.blue" , self.color9 .blue());
        map.insert("color10.blue", self.color10.blue());
        map.insert("color11.blue", self.color11.blue());
        map.insert("color12.blue", self.color12.blue());
        map.insert("color13.blue", self.color13.blue());
        map.insert("color14.blue", self.color14.blue());
        map.insert("color15.blue", self.color15.blue());
        map.insert("foreground.blue", self.foreground.blue());
        map.insert("background.blue", self.background.blue());

        map
    }
}
