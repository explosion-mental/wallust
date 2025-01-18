//! Template stuff, definitions and how it's parsed
use std::fs::read_to_string;
use std::path::Path;
use std::collections::HashMap;

use crate::{
    colors::Colors,
    config::Fields,
    palettes::Palette,
    backends::Backend,
    colorspaces::ColorSpace,
};

use anyhow::Result;
use owo_colors::OwoColorize;
use minijinja::{Environment, context};

pub mod pywal;
pub mod jinja2;
use jinja2::{
    jinja_env,
    jinja_update_alpha,
    minijinja_err_chain,
};

pub struct TemplateFields<'a> {
    pub alpha: u8,
    pub backend: &'a Backend,
    pub palette: &'a Palette,
    pub colorspace: &'a ColorSpace,
    pub image_path: &'a str,
    pub colors: &'a Colors,
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
pub fn file_render(env: &mut Environment, file: &Path, target_path: &Path, pywal: bool, values: &TemplateFields) -> Result<(), String> {
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
        jinja_update_alpha(env, values.alpha);
        let name = file.display().to_string();
        let v = minijinja::Value::from(values);

        //env.add_template(&name, &file_content);
        // env.add_template_owned(name, file_content).map_err(minijinja_err_chain)?;
        //
        // let t = env.get_template(&file.display().to_string()).map_err(minijinja_err_chain)?;
        // t.render(v)

        env.render_named_str(&name, &file_content, v)
            .map_err(minijinja_err_chain)?
    } else {
        pywal::render(&file_content, values)
            .map_err(|err| format!("Error while rendering '{filename}': {err}"))?
    };

    // map io::Errors into a writeable one (String) ((maybe this is how anyhow werks?))
    std::fs::write(target_path, rendered)
        .map_err(|err| format!("Error while writting to {}: {err}", target_path.display()))
}

/// Writes `template`s into `target`s. Given the many possibilities of I/O errors, template errors,
/// user typos, etc. Most errors are reported to stderr, and ignored to `continue` with the other
/// entries.
pub fn write_template(config_dir: &Path, templates_header: &HashMap<String, Fields>, values: &TemplateFields, quiet: bool, env_vars: bool) -> Result<()> {

    let mut jinjaenv = jinja_env();
    //XXX loader makes avaliable the (easy) use of `import` and such
    jinjaenv.set_loader(minijinja::path_loader(config_dir));


    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path

    for (name, fields) in templates_header {
        // facilitates strings printing
        let name = name.bold();
        let warn = "W".red();
        let warn = warn.bold();

        //root path for the template file
        let path = config_dir.join(&fields.template);

        //root path for the target file (requires interpret `~` for home)
        //XXX on `shellexpand`, think about using `::full()` to support env vars. Seems a bit sketchy/sus
        let env = match env_vars {
            true => shellexpand::full(&fields.target)?,
            false => shellexpand::tilde(&fields.target),
        };

        // pretty printing of the path
        let target = env.italic();

        let target_path = Path::new(env.as_ref());

        let pywal = fields.pywal.unwrap_or(false);

        if !path.is_dir() { // normal file
            if let Err(err) = file_render(&mut jinjaenv, &path, target_path, pywal, values) {
                eprintln!("[{warn}] {name}: {err}");
                continue;
            }
            if ! quiet { println!("  * Templated {name} to '{target}'"); }
        } else {
            if ! quiet { println!("  * Templating {name}: directory at '{}'", path.display().italic()); }
            // read directory, encapsulating this into a function and then calling this recursively handle the `recursive` field?
            for i in path.read_dir()? {
                let i = i?;

                let f = &i.file_name();

                let target_path = target_path.join(f);

                if let Err(err) = file_render(&mut jinjaenv, &path.join(f), &target_path, pywal, values) {
                    eprintln!("[{warn}] {name}: {err}");
                    continue;
                }
                if ! quiet { println!("     + {name} {} to '{target}'", &i.path().display(), target = target_path.display().italic()); }
            }
        }
    }

    Ok(())
}

impl From<&TemplateFields<'_>> for minijinja::Value {
    fn from(values: &TemplateFields<'_>) -> Self {
        let c = &values.colors;
        let v = minijinja::Value::from_serialize(c);

        context! {
            ..v,
            ..context! {
                alpha      => values.alpha,
                cursor     => c.foreground,
                palette    => values.palette,
                wallpaper  => values.image_path,
                backend    => values.backend,
                colorspace => values.colorspace,
                colors     => c.into_iter().map(|x| x.to_string()).collect::<Vec<String>>(),
            }
        }

    }
}

/// This is used to represent HEXA values, but only the alpha part.
/// Alpha doesn't go as far as 255, only up to a 100, so simple fmt like {:0X} won't do the job.
/// Since [`Myrgb`] type doesn't implement alpha by itself, alpha it's represented separetly.
/// list of hexadecimal alpha values
/// refs:
/// - <https://gist.github.com/lopspower/03fb1cc0ac9f32ef38f4>
/// - <https://net-informations.com/q/web/trans.html>
fn alpha_hexa(input: usize) -> Result<String, &'static str> {
    let alphas_hex = [ "00", "03", "05", "08", "0A", "0D", "0F", "12", "14", "17", "1A", "1C", "1F", "21", "24", "26", "29", "2B", "2E", "30", "33", "36", "38", "3B", "3D", "40", "42", "45", "47", "4A", "4D", "4F", "52", "54", "57", "59", "5C", "5E", "61", "63", "66", "69", "6B", "6E", "70", "73", "75", "78", "7A", "7D", "80", "82", "85", "87", "8A", "8C", "8F", "91", "94", "96", "99", "9C", "9E", "A1", "A3", "A6", "A8", "AB", "AD", "B0", "B3", "B5", "B8", "BA", "BD", "BF", "C2", "C4", "C7", "C9", "CC", "CF", "D1", "D4", "D6", "D9", "DB", "DE", "E0", "E3", "E6", "E8", "EB", "ED", "F0", "F2", "F5", "F7", "FA", "FC", "FF", ];
    let ret = alphas_hex.get(input);
    match ret {
        Some(s) => Ok(s.to_string()),
        None => Err("Input should be in the range of 0 to 100.")
    }
}

/// This is only used for pywal
impl TemplateFields<'_> {
pub fn to_hash<'a>(&self) -> HashMap<&'a str, String> {
    let mut map = HashMap::new();
    let alpha = self.alpha;
    let col = self.colors;
    let alpha_hex = alpha_hexa(alpha as usize).expect("CANNOT OVERFLOW, validation with clap 0..=100");
    let alpha_dec = f32::from(alpha) / 100.0;

    map.insert("wallpaper", self.image_path.into()); //full path to the image
    map.insert("alpha", alpha.to_string());
    map.insert("alpha_dec", format!("{alpha_dec:.2}"));
    map.insert("alpha_hex", alpha_hex);

    // Include backend, colorspace and filter (palette)
    map.insert("backend", self.backend.to_string());
    map.insert("colorspace", self.colorspace.to_string());
    map.insert("palette", self.palette.to_string());

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

    map
}
}
