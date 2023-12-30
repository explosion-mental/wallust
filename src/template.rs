//! Template stuff, definitions and how it's parsed
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;

use crate::config::Config;
use crate::config::Entries;
use crate::colors::Colors;

use anyhow::Result;
use owo_colors::OwoColorize;

/// Writes `template`s into `target`s. Given the many possibilities of I/O errors, template errors,
/// user typos, etc. Most errors are reported to stderr, and ignored to `continue` with the other
/// entries.
pub fn write_template(conf: &Config, image_path: &Path, entries: &[Entries], values: &Colors, quiet: bool) -> Result<()> {
    let config = &conf.dir;

    let warn = "W".red();
    let warn = warn.bold();

    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path
    for e in entries {
        let path = config.join(&e.template).display().to_string();

        let file_template = match read_to_string(&path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[{warn}] Skipping {path}: {e}");
                continue;
            }
        };

        let target = &e.target;
        let file_content = file_template;

        //XXX on `shellexpand`, think about using `::full()` to support env vars. Seems a bit sketchy/sus
        let target_file = shellexpand::tilde(target);

        if ! quiet { println!("  * Templating: {}", e.template); }

        if let Some(p) = Path::new(target_file.as_ref()).parent() {
            if let Err(e) = std::fs::create_dir_all(p) {
                eprintln!("[{warn}] Failed to create parent directories from {target}: {e}");
                continue;
            }
        } else {
            eprintln!("[{warn}] Failed to find file parent from {target}");
            continue;
        }

        let rendered = if e.new_engine.unwrap_or(false) {
            far::find_with_mode(file_content, far::Mode::AllowMissing)?.replace(&Myt { cols: values, img: &image_path, conf })
        } else {
            new_string_template::template::Template::new(file_content).render_nofail(&values.to_hash(image_path, conf))
        };

        let mut buffer = match File::create(target_file.as_ref()) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[{warn}] Failed to create file {target}: {e}");
                continue;
            }
        };

        if let Err(e) = buffer.write_all(rendered.as_bytes()) {
            eprintln!("[{warn}] Failed to write to file {target}: {e}");
            continue;
        }

        if ! quiet { println!("      Created: {} ... OK", target); }
    }
    Ok(())
}

struct Myt<'a> {
    cols: &'a Colors,
    img: &'a std::path::Path,
    conf: &'a Config,
}

impl<'a> far::Render for Myt<'a> {
    fn render(&self) -> HashMap<&'static str, String> {
        to_hash(&self.cols, &self.img, &self.conf)

    }
    fn keys() -> Box<dyn Iterator<Item = &'static str>> {
        Box::new([
            "wallpaper",
            "alpha",
            "alpha_dec",
            "backend",
            "colorspace",
            "filter",
            "color0" ,
            "color1" ,
            "color2" ,
            "color3" ,
            "color4" ,
            "color5" ,
            "color6" ,
            "color7" ,
            "color8" ,
            "color9" ,
            "color10",
            "color11",
            "color12",
            "color13",
            "color14",
            "color15",
            "cursor",
            "foreground",
            "background",
            "color0.rgb" ,
            "color1.rgb" ,
            "color2.rgb" ,
            "color3.rgb" ,
            "color4.rgb" ,
            "color5.rgb" ,
            "color6.rgb" ,
            "color7.rgb" ,
            "color8.rgb" ,
            "color9.rgb" ,
            "color10.rgb",
            "color11.rgb",
            "color12.rgb",
            "color13.rgb",
            "color14.rgb",
            "color15.rgb",
            "cursor.rgb",
            "foreground.rgb",
            "background.rgb",
            "color0.rgba" ,
            "color1.rgba" ,
            "color2.rgba" ,
            "color3.rgba" ,
            "color4.rgba" ,
            "color5.rgba" ,
            "color6.rgba" ,
            "color7.rgba" ,
            "color8.rgba" ,
            "color9.rgba" ,
            "color10.rgba",
            "color11.rgba",
            "color12.rgba",
            "color13.rgba",
            "color14.rgba",
            "color15.rgba",
            "cursor.rgba",
            "foreground.rgba",
            "background.rgba",
            "color0.xrgba" ,
            "color1.xrgba" ,
            "color2.xrgba" ,
            "color3.xrgba" ,
            "color4.xrgba" ,
            "color5.xrgba" ,
            "color6.xrgba" ,
            "color7.xrgba" ,
            "color8.xrgba" ,
            "color9.xrgba" ,
            "color10.xrgba",
            "color11.xrgba",
            "color12.xrgba",
            "color13.xrgba",
            "color14.xrgba",
            "color15.xrgba",
            "cursor.xrgba",
            "foreground.xrgba",
            "background.xrgba",
            "color0.strip" ,
            "color1.strip" ,
            "color2.strip" ,
            "color3.strip" ,
            "color4.strip" ,
            "color5.strip" ,
            "color6.strip" ,
            "color7.strip" ,
            "color8.strip" ,
            "color9.strip" ,
            "color10.strip",
            "color11.strip",
            "color12.strip",
            "color13.strip",
            "color14.strip",
            "color15.strip",
            "cursor.strip",
            "foreground.strip",
            "background.strip",
            "color0.red" ,
            "color1.red" ,
            "color2.red" ,
            "color3.red" ,
            "color4.red" ,
            "color5.red" ,
            "color6.red" ,
            "color7.red" ,
            "color8.red" ,
            "color9.red" ,
            "color10.red",
            "color11.red",
            "color12.red",
            "color13.red",
            "color14.red",
            "color15.red",
            "cursor.red",
            "foreground.red",
            "background.red",
            "color0.green" ,
            "color1.green" ,
            "color2.green" ,
            "color3.green" ,
            "color4.green" ,
            "color5.green" ,
            "color6.green" ,
            "color7.green" ,
            "color8.green" ,
            "color9.green" ,
            "color10.green",
            "color11.green",
            "color12.green",
            "color13.green",
            "color14.green",
            "color15.green",
            "cursor.green",
            "foreground.green",
            "background.green",
            "color0.blue" ,
            "color1.blue" ,
            "color2.blue" ,
            "color3.blue" ,
            "color4.blue" ,
            "color5.blue" ,
            "color6.blue" ,
            "color7.blue" ,
            "color8.blue" ,
            "color9.blue" ,
            "color10.blue",
            "color11.blue",
            "color12.blue",
            "color13.blue",
            "color14.blue",
            "color15.blue",
            "cursor.blue",
            "foreground.blue",
            "background.blue",
            ].into_iter())
    }

}

pub fn to_hash<'a>(col: &Colors, image_path: &Path, conf: &Config) -> HashMap<&'a str, String> {
    let mut map = HashMap::new();
    let alpha = conf.alpha.unwrap_or(100);
    // make sure to display the absolute path of the wallpaper
    let p = std::fs::canonicalize(image_path).expect("PATH EXIST, validation from clap");

    //XXX instead of multiple `.method()` maybe using enums and match with a single method

    //full path to the image
    map.insert("wallpaper", p.display().to_string());
    map.insert("alpha", alpha.to_string());
    map.insert("alpha_dec", format!("{:.2}", f32::from(alpha) / 100.0 ));
    //map.insert("alpha_hex", format!("{:.02x}",  ((f32::from(alpha) / 100.0)  * 255.0) as i32 ));

    // Include backend, colorspace and filter
    map.insert("backend", conf.backend.to_string());
    map.insert("colorspace", conf.color_space.to_string());
    map.insert("filter", conf.filter.to_string());

    // normal output `#EEEEEE`
    map.insert("color0" , col.color0 .to_string());
    map.insert("color1" , col.color1 .to_string());
    map.insert("color2" , col.color2 .to_string());
    map.insert("color3" , col.color3 .to_string());
    map.insert("color4" , col.color4 .to_string());
    map.insert("color5" , col.color5 .to_string());
    map.insert("color6" , col.color6 .to_string());
    map.insert("color7" , col.color7 .to_string());
    map.insert("color8" , col.color8 .to_string());
    map.insert("color9" , col.color9 .to_string());
    map.insert("color10", col.color10.to_string());
    map.insert("color11", col.color11.to_string());
    map.insert("color12", col.color12.to_string());
    map.insert("color13", col.color13.to_string());
    map.insert("color14", col.color14.to_string());
    map.insert("color15", col.color15.to_string());
    map.insert("cursor", col.foreground.to_string());
    map.insert("foreground", col.foreground.to_string());
    map.insert("background", col.background.to_string());

    //.rgb output `235,235,235`
    map.insert("color0.rgb" , col.color0 .rgb());
    map.insert("color1.rgb" , col.color1 .rgb());
    map.insert("color2.rgb" , col.color2 .rgb());
    map.insert("color3.rgb" , col.color3 .rgb());
    map.insert("color4.rgb" , col.color4 .rgb());
    map.insert("color5.rgb" , col.color5 .rgb());
    map.insert("color6.rgb" , col.color6 .rgb());
    map.insert("color7.rgb" , col.color7 .rgb());
    map.insert("color8.rgb" , col.color8 .rgb());
    map.insert("color9.rgb" , col.color9 .rgb());
    map.insert("color10.rgb", col.color10.rgb());
    map.insert("color11.rgb", col.color11.rgb());
    map.insert("color12.rgb", col.color12.rgb());
    map.insert("color13.rgb", col.color13.rgb());
    map.insert("color14.rgb", col.color14.rgb());
    map.insert("color15.rgb", col.color15.rgb());
    map.insert("cursor.rgb", col.foreground.rgb());
    map.insert("foreground.rgb", col.foreground.rgb());
    map.insert("background.rgb", col.background.rgb());

    //.rgba output `235,235,235,1.0`
    map.insert("color0.rgba" , col.color0 .rgba());
    map.insert("color1.rgba" , col.color1 .rgba());
    map.insert("color2.rgba" , col.color2 .rgba());
    map.insert("color3.rgba" , col.color3 .rgba());
    map.insert("color4.rgba" , col.color4 .rgba());
    map.insert("color5.rgba" , col.color5 .rgba());
    map.insert("color6.rgba" , col.color6 .rgba());
    map.insert("color7.rgba" , col.color7 .rgba());
    map.insert("color8.rgba" , col.color8 .rgba());
    map.insert("color9.rgba" , col.color9 .rgba());
    map.insert("color10.rgba", col.color10.rgba());
    map.insert("color11.rgba", col.color11.rgba());
    map.insert("color12.rgba", col.color12.rgba());
    map.insert("color13.rgba", col.color13.rgba());
    map.insert("color14.rgba", col.color14.rgba());
    map.insert("color15.rgba", col.color15.rgba());
    map.insert("cursor.rgba", col.foreground.rgba());
    map.insert("foreground.rgba", col.foreground.rgba());
    map.insert("background.rgba", col.background.rgba());

    //.xrgba output `ee/ee/ee/ff`
    map.insert("color0.xrgba" , col.color0 .xrgba());
    map.insert("color1.xrgba" , col.color1 .xrgba());
    map.insert("color2.xrgba" , col.color2 .xrgba());
    map.insert("color3.xrgba" , col.color3 .xrgba());
    map.insert("color4.xrgba" , col.color4 .xrgba());
    map.insert("color5.xrgba" , col.color5 .xrgba());
    map.insert("color6.xrgba" , col.color6 .xrgba());
    map.insert("color7.xrgba" , col.color7 .xrgba());
    map.insert("color8.xrgba" , col.color8 .xrgba());
    map.insert("color9.xrgba" , col.color9 .xrgba());
    map.insert("color10.xrgba", col.color10.xrgba());
    map.insert("color11.xrgba", col.color11.xrgba());
    map.insert("color12.xrgba", col.color12.xrgba());
    map.insert("color13.xrgba", col.color13.xrgba());
    map.insert("color14.xrgba", col.color14.xrgba());
    map.insert("color15.xrgba", col.color15.xrgba());
    map.insert("cursor.xrgba", col.foreground.xrgba());
    map.insert("foreground.xrgba", col.foreground.xrgba());
    map.insert("background.xrgba", col.background.xrgba());

    //.strip output `EEEEEE`
    map.insert("color0.strip" , col.color0 .strip());
    map.insert("color1.strip" , col.color1 .strip());
    map.insert("color2.strip" , col.color2 .strip());
    map.insert("color3.strip" , col.color3 .strip());
    map.insert("color4.strip" , col.color4 .strip());
    map.insert("color5.strip" , col.color5 .strip());
    map.insert("color6.strip" , col.color6 .strip());
    map.insert("color7.strip" , col.color7 .strip());
    map.insert("color8.strip" , col.color8 .strip());
    map.insert("color9.strip" , col.color9 .strip());
    map.insert("color10.strip", col.color10.strip());
    map.insert("color11.strip", col.color11.strip());
    map.insert("color12.strip", col.color12.strip());
    map.insert("color13.strip", col.color13.strip());
    map.insert("color14.strip", col.color14.strip());
    map.insert("color15.strip", col.color15.strip());
    map.insert("cursor.strip", col.foreground.strip());
    map.insert("foreground.strip", col.foreground.strip());
    map.insert("background.strip", col.background.strip());

    //.red output `235`
    map.insert("color0.red" , col.color0 .red());
    map.insert("color1.red" , col.color1 .red());
    map.insert("color2.red" , col.color2 .red());
    map.insert("color3.red" , col.color3 .red());
    map.insert("color4.red" , col.color4 .red());
    map.insert("color5.red" , col.color5 .red());
    map.insert("color6.red" , col.color6 .red());
    map.insert("color7.red" , col.color7 .red());
    map.insert("color8.red" , col.color8 .red());
    map.insert("color9.red" , col.color9 .red());
    map.insert("color10.red", col.color10.red());
    map.insert("color11.red", col.color11.red());
    map.insert("color12.red", col.color12.red());
    map.insert("color13.red", col.color13.red());
    map.insert("color14.red", col.color14.red());
    map.insert("color15.red", col.color15.red());
    map.insert("cursor.red", col.foreground.red());
    map.insert("foreground.red", col.foreground.red());
    map.insert("background.red", col.background.red());

    //.green output `235`
    map.insert("color0.green" , col.color0 .green());
    map.insert("color1.green" , col.color1 .green());
    map.insert("color2.green" , col.color2 .green());
    map.insert("color3.green" , col.color3 .green());
    map.insert("color4.green" , col.color4 .green());
    map.insert("color5.green" , col.color5 .green());
    map.insert("color6.green" , col.color6 .green());
    map.insert("color7.green" , col.color7 .green());
    map.insert("color8.green" , col.color8 .green());
    map.insert("color9.green" , col.color9 .green());
    map.insert("color10.green", col.color10.green());
    map.insert("color11.green", col.color11.green());
    map.insert("color12.green", col.color12.green());
    map.insert("color13.green", col.color13.green());
    map.insert("color14.green", col.color14.green());
    map.insert("color15.green", col.color15.green());
    map.insert("cursor.green", col.foreground.green());
    map.insert("foreground.green", col.foreground.green());
    map.insert("background.green", col.background.green());

    //.blue output `235`
    map.insert("color0.blue" , col.color0 .blue());
    map.insert("color1.blue" , col.color1 .blue());
    map.insert("color2.blue" , col.color2 .blue());
    map.insert("color3.blue" , col.color3 .blue());
    map.insert("color4.blue" , col.color4 .blue());
    map.insert("color5.blue" , col.color5 .blue());
    map.insert("color6.blue" , col.color6 .blue());
    map.insert("color7.blue" , col.color7 .blue());
    map.insert("color8.blue" , col.color8 .blue());
    map.insert("color9.blue" , col.color9 .blue());
    map.insert("color10.blue", col.color10.blue());
    map.insert("color11.blue", col.color11.blue());
    map.insert("color12.blue", col.color12.blue());
    map.insert("color13.blue", col.color13.blue());
    map.insert("color14.blue", col.color14.blue());
    map.insert("color15.blue", col.color15.blue());
    map.insert("cursor.blue", col.foreground.blue());
    map.insert("foreground.blue", col.foreground.blue());
    map.insert("background.blue", col.background.blue());

    map
}


impl Colors {
    pub fn to_hash(&self, image_path: &Path, conf: &Config) -> HashMap<&str, String> {
        to_hash(self, image_path, conf)
    }
}
