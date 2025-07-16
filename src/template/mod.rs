//! Template stuff, definitions and how it's parsed
use std::fs::{create_dir_all, read_to_string, write};
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
use minijinja::Environment;
use walkdir::WalkDir;

pub mod pywal;
pub mod jinja2;
pub mod colors;

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

impl TemplateFields<'_> {
    pub fn render_jinja(&self, env: &mut Environment, file: &Path, target_path: &Path) -> Result<(), String> {
        let filename = file.display();
        let filename = filename.italic();
        let targetname = target_path.display();
        let targetname = targetname.italic();

        let file_content = read_to_string(file)
            .map_err(|err| format!("Reading {filename} failed: {err}"))?;

        // First find if the parent exists at all before rendering
        match target_path.parent() {
            Some(s) => create_dir_all(s)
                .map_err(|err| format!("Failed to create parent directories from {targetname}: {err}"))?,
            None => return Err(format!("Failed to find file parent from {targetname}")),
        };

        // Template/render the file_contents
        jinja_update_alpha(env, self.alpha);
        let name = file.display().to_string();
        let v = minijinja::Value::from(self);

        let rendered = env.render_named_str(&name, &file_content, v).map_err(minijinja_err_chain)?;

        // map io::Errors into a writeable one (String) ((maybe this is how anyhow werks?))
        write(target_path, rendered)
            .map_err(|err| format!("Error while writting to {targetname}: {err}"))

    }

    pub fn render_pywal(&self, file: &Path, target_path: &Path) -> Result<(), String> {
        let filename = file.display();
        let filename = filename.italic();
        let targetname = target_path.display();
        let targetname = targetname.italic();

        let file_content = read_to_string(file)
            .map_err(|err| format!("Reading {filename} failed: {err}"))?;

        match target_path.parent() {
            Some(s) => create_dir_all(s)
                .map_err(|err| format!("Failed to create parent directories from {targetname}: {err}"))?,
            None => return Err(format!("Failed to find file parent from {targetname}")),
        };

        let rendered = pywal::render(&file_content, self).map_err(|err| format!("Error while rendering '{filename}': {err}"))?;

        write(target_path, rendered)
            .map_err(|err| format!("Error while writting to {targetname}: {err}"))

    }

    /// Convert to a hash that I can later `.get()`
    pub fn to_hash<'a>(&self) -> HashMap<&'a str, String> {
        let mut map = HashMap::new();
        let alpha = self.alpha;
        let col = self.colors;
        let alpha_hex = alpha_hexa(alpha as usize).expect("CANNOT OVERFLOW, validation with clap 0..=100");
        let alpha_dec = f32::from(alpha) / 100.0;
        let alpha_dec = if alpha % 10 == 0 { format!("{alpha_dec:.1}") } else { format!("{alpha_dec:.2}") };

        map.insert("wallpaper", self.image_path.into()); //full path to the image
        map.insert("alpha", alpha.to_string());
        map.insert("alpha_dec", alpha_dec.to_string() );
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
        map.insert("cursor", col.cursor.to_string());
        map.insert("foreground", col.foreground.to_string());
        map.insert("background", col.background.to_string());

        map
    }
}


/// Writes `template`s into `target`s. Given the many possibilities of I/O errors, template errors,
/// user typos, etc. Most errors are reported to stderr, and ignored to `continue` with the other
/// entries.
pub fn write_template(config_dir: &Path, templates_header: &HashMap<String, Fields>, values: &TemplateFields, quiet: bool, env_vars: bool) -> Result<()> {
    // let has_pywal = templates_header.iter().any(|x| x.1.pywal == Some(true));
    // let mut jinjaenv = if has_pywal { Some(jinja_env()) } else { None };
    let mut jinjaenv = jinja_env();

    //XXX loader makes avaliable the (easy) use of `import` and such
    jinjaenv.set_loader(minijinja::path_loader(config_dir));

    let warn = "W".red();
    let warn = warn.bold();
    let ast = "*".blue();
    let ast = ast.bold();

    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated file to entry.path
    for (name, fields) in templates_header {
        //root path for the target file (requires interpret `~` for home)
        let env = if env_vars { shellexpand::full(&fields.target)? } else { shellexpand::tilde(&fields.target) };

        //root path for the template file
        let path = config_dir.join(&fields.template);

        // pretty printing
        let name = name.bold();
        let target = env.italic();

        let target_path = Path::new(env.as_ref());

        let pywal = fields.pywal.unwrap_or(false);

        if !path.is_dir() { // normal file
            let render = if pywal { values.render_pywal(&path, target_path) } else { values.render_jinja(&mut jinjaenv, &path, target_path) };
            if let Err(err) = render {
                eprintln!("[{warn}] {name}: {err}");
                continue;
            }

            if ! quiet { println!(" {ast} Templated {name} to '{target}'"); }

        } else {
            if ! quiet { println!(" {ast} Templating {name}: directory at '{}'", path.display().italic()); }

            match fields.max_depth {
                Some(d) => WalkDir::new(&path).max_depth(d.into()),
                None => WalkDir::new(&path),
            }
                .into_iter()
                .filter_map(|f| f.ok())
                .filter(|f| f.file_type().is_file())
                .for_each(|f| {
                    let f = f.path();
                    // copy dir tree
                    let relative = f.strip_prefix(&path).expect("strip_prefix() failed"); //XXX
                    let target_path = target_path.join(relative);

                    let render = if pywal { values.render_pywal(&f, &target_path) } else { values.render_jinja(&mut jinjaenv, &f, &target_path) };
                    if let Err(err) = render {
                        eprintln!("[{warn}] {name}: {err}");
                        return;
                    }
                    if ! quiet { println!("   + {name} {} to '{}'", &f.display(), target_path.display().italic()); }
                });
        }
    }

    Ok(())
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
