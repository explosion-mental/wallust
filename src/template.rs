//! Template stuff, definitions and how it's parsed
use std::fs::read_to_string;
use std::path::Path;
use std::collections::HashMap;

use crate::config::Config;
use crate::colors::Colors;
use crate::colors::Myrgb;

use anyhow::Result;
use owo_colors::OwoColorize;
use minijinja::{Environment, context};
use minijinja::value::ViaDeserialize;

macro_rules! jinjafn {
    ($var:expr, $func_name:ident) => {
        fn $func_name(value: ViaDeserialize<Myrgb>) -> String { Myrgb::$func_name(&value) }
        $var.add_filter(stringify!($func_name), $func_name);
    };
    ($var:expr, tostr => $func_name:ident) => {
        fn $func_name(value: ViaDeserialize<Myrgb>) -> String { Myrgb::$func_name(&value).to_string() }
        $var.add_filter(stringify!($func_name), $func_name);
    };

    ($var:expr, $func_name:ident, $arg:ty) => {
        fn $func_name(value: ViaDeserialize<Myrgb>, other: $arg) -> String { Myrgb::$func_name(&value, other) }
        $var.add_filter(stringify!($func_name), $func_name);
    };
    ($var:expr, tostr => $func_name:ident, $arg:ty) => {
        fn $func_name(value: ViaDeserialize<Myrgb>, other: $arg) -> String { Myrgb::$func_name(&value, other).to_string() }
        $var.add_filter(stringify!($func_name), $func_name);
    };
    ($var:expr, $func_name:ident, deref => $arg:ty) => {
        fn $func_name(value: ViaDeserialize<Myrgb>, other: $arg) -> String { Myrgb::$func_name(&value, *other) }
        $var.add_filter(stringify!($func_name), $func_name);
    };
    ($var:expr, tostr => $func_name:ident, deref => $arg:ty) => {
        fn $func_name(value: ViaDeserialize<Myrgb>, other: $arg) -> String { Myrgb::$func_name(&value, *other).to_string() }
        $var.add_filter(stringify!($func_name), $func_name);
    };
}

/// Recommended way to chain errors
/// ref: <https://docs.rs/minijinja/latest/minijinja/struct.Error.html>
fn minijinja_err_chain(err: minijinja::Error) -> String {
    let mut err = &err as &dyn std::error::Error;
    let mut s = String::from(&format!("Could not render template: {err:#}"));

    // get to the source, if there are more.
    while let Some(next_err) = err.source() {
        s.push('\n');
        s.push_str(&format!("Caused by: {next_err:#}"));
        err = next_err;
    }
    s
}

/// Render the template `file` provided and write it to `target_path`.
/// `.map_err` is used to append friendly "Reading 'file' failed" or the like,
/// since we don't care about handling all possible io::Errors
// TODO there's gonna be trouble harcoding:
// //with jinja:
// let mut env = Environment::new(); //preload jinja enviroment
// ... // set jinja functions
// file_render(&env, ..) //pass env as reference,
// //with pywal(new_string):
// let values = values.to_hash(..); //preload hashmap, so it doesn't create a new one for every iteration
// file_render(&values, ..);
//
// Maybe a solution is something like:
// let test = templates.iter().any(|x| x.pywal == Some(true));
// Then create env or the hasmap as an option, and `.expect` to open it an pass it as file_render():
// 1. If it's None, it will never reach expect.
// 2. If it's Some, it will always be true an a valid value.
pub fn file_render(file: &Path, target_path: &Path, pywal: bool, conf: &Config, image_path: &str, values: &Colors) -> Result<(), String> {
    let filename = file.display();
    let filename = filename.italic();

    let file_content = read_to_string(file)
        .map_err(|err| format!("Reading {filename} failed: {err}"))?;

    // First find if the parent exists at all before rendering
    match target_path.parent() {
       Some(s) => std::fs::create_dir_all(s)
           .map_err(|err| format!("Failed to create parent directories from {}: {err}", target_path.display().italic()))?,
       None => return Err(format!("Failed to find file parent from {}", target_path.display().italic())),
    };

    // Template/render the file_contents
    let rendered = if ! pywal {
        let env = jinja_env();
        let variables = jinja_values(values, image_path, conf.palette, conf.backend, conf.color_space);

        let name = file.display().to_string();

        env.render_named_str(&name, &file_content, variables)
            .map_err(minijinja_err_chain)?
    } else {
        new_string_template::template::Template::new(file_content)
            // this regex is even better than pywal, doesn't match new lines :3
            // <https://regex101.com/r/AgVXKJ/1>
            .with_regex(&regex::Regex::new(r"\{(\S+?)\}").expect("correct tested regex"))
            .render(&values.to_hash(image_path, conf))
            .map_err(|err| format!("Error while rendering '{filename}': {err}"))?
    };

    // map io::Errors into a writeable one (String) ((maybe this is how anyhow werks?))
    std::fs::write(target_path, rendered)
        .map_err(|err| format!("Error while writting to {}: {err}", target_path.display()))
}

/// Writes `template`s into `target`s. Given the many possibilities of I/O errors, template errors,
/// user typos, etc. Most errors are reported to stderr, and ignored to `continue` with the other
/// entries.
pub fn write_template(conf: &Config, image_path: &str, values: &Colors, quiet: bool) -> Result<()> {
    let init = format!("[{info}] {t}: ", info = "I".blue().bold(), t = "templates".magenta().bold());
    let config = &conf.dir;

    let templates_header = match &conf.templates {
        Some(s) => {
            if ! quiet { println!("{init}Writing templates.."); }
            s
        },
        None => {
            if ! quiet { println!("{init}No templates found"); }
            return Ok(())
        },
    };

    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path
    for e in templates_header {

        //root path for the template file
        let path = config.join(&e.1.template);

        //root path for the target file (requires interpret `~` for home)
        //XXX on `shellexpand`, think about using `::full()` to support env vars. Seems a bit sketchy/sus
        let env = shellexpand::tilde(&e.1.target);
        let target_path = Path::new(env.as_ref());

        // for printing
        let name = e.0.bold();
        let target = &e.1.target.italic();
        let warn = "W".red();
        let warn = warn.bold();

        // TODO handle `.recursive` field
        if path.is_dir() {
            if ! quiet { println!("  * Templating {name}: directory at '{}'", path.display().italic()); }

            // read directory, encapsulating this into a function and then calling this recursively handle the `recursive` field?
            for i in path.read_dir()? {
                let i = i?;

                //println!("{i:?}");
                let f = &i.file_name();
                //println!(" THIS IS FILE {f:?}");

                let target_path = target_path.join(f);

                let target = target_path.display();
                let target = target.italic();

                if ! quiet { println!("     + {name} {} to '{target}'", &i.path().display()); }

                if let Err(err) = file_render(&path.join(f), &target_path, e.1.pywal.unwrap_or(false), conf, image_path, values) {
                    eprintln!("[{warn}] {name}: {err}");
                    continue;
                }
            }
        } else {
            if ! quiet { println!("  * Templated {name} to '{target}'"); }
            if let Err(err) = file_render(&path, target_path, e.1.pywal.unwrap_or(false), conf, image_path, values) {
                eprintln!("[{warn}] {name}: {err}");
                continue;
            }
        }
    }

    Ok(())
}

pub fn jinja_env<'a>() -> Environment<'a> {
        let mut env = Environment::new();

        env.set_keep_trailing_newline(true); // keep the template file intact

        //filters
        jinjafn!(env, rgb);
        jinjafn!(env, xrgb);
        jinjafn!(env, strip);
        jinjafn!(env, red);
        jinjafn!(env, green);
        jinjafn!(env, blue);
        jinjafn!(env, tostr => complementary);
        jinjafn!(env, tostr => blend, deref => ViaDeserialize<Myrgb>);
        jinjafn!(env, tostr => lighten, f32);
        jinjafn!(env, tostr => darken, f32);
        jinjafn!(env, tostr => saturate, f32);
        env
}

pub fn jinja_values(
    values: &Colors,
    image_path: &str,
    palette: crate::palettes::Palette,
    backend: crate::backends::Backend,
    colorspace: crate::colorspaces::ColorSpace,
) -> minijinja::Value {
    let v = minijinja::Value::from_serializable(&values);
    context! {
        ..v,
        ..context! {
            cursor     => values.foreground,
            palette    => palette,
            wallpaper  => image_path,
            backend    => backend,
            colorspace => colorspace,
            colors     => values.into_iter().map(|x| x.to_string()).collect::<String>(),
        }
    }
}


/// hash values
fn to_hash<'a>(col: &Colors, image_path: &str, conf: &Config) -> HashMap<&'a str, String> {
    let mut map = HashMap::new();
    let alpha = conf.alpha.unwrap_or(100);
    // list of hexadecimal alpha values https://gist.github.com/lopspower/03fb1cc0ac9f32ef38f4
    let alphas_dec = [ "00", "03", "05", "08", "0A", "0D", "0F", "12", "14", "17", "1A", "1C", "1F", "21", "24", "26", "29", "2B", "2E", "30", "33", "36", "38", "3B", "3D", "40", "42", "45", "47", "4A", "4D", "4F", "52", "54", "57", "59", "5C", "5E", "61", "63", "66", "69", "6B", "6E", "70", "73", "75", "78", "7A", "7D", "80", "82", "85", "87", "8A", "8C", "8F", "91", "94", "96", "99", "9C", "9E", "A1", "A3", "A6", "A8", "AB", "AD", "B0", "B3", "B5", "B8", "BA", "BD", "BF", "C2", "C4", "C7", "C9", "CC", "CF", "D1", "D4", "D6", "D9", "DB", "DE", "E0", "E3", "E6", "E8", "EB", "ED", "F0", "F2", "F5", "F7", "FA", "FC", "FF", ];

    //XXX instead of multiple `.method()` maybe using enums and match with a single method

    //full path to the image
    map.insert("wallpaper", image_path.into());
    map.insert("alpha", alpha.to_string());
    map.insert("alpha_dec", format!("{:.2}", f32::from(alpha) / 100.0 ));
    map.insert("alpha_hex", alphas_dec.get(alpha as usize).expect("CANNOT OVERFLOW, validation with clap 0..=100").to_string());

    // Include backend, colorspace and filter (palette)
    map.insert("backend", conf.backend.to_string());
    map.insert("colorspace", conf.color_space.to_string());
    map.insert("palette", conf.palette.to_string());

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
    pub fn to_hash(&self, image_path: &str, conf: &Config) -> HashMap<&str, String> {
        to_hash(self, image_path, conf)
    }
}
