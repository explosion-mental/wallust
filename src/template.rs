//! Template stuff, definitions and how it's parsed
use std::fs::read_to_string;
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;

use crate::{
    colors::{ Colors, Myrgb },
    config::Fields,
    palettes::Palette,
    backends::Backend,
    colorspaces::ColorSpace,
};

use anyhow::Result;
use owo_colors::OwoColorize;
use minijinja::{Environment, context};
use minijinja::value::ViaDeserialize;

use palette::{
    Darken, Lighten, IntoColor, Saturate,
    Srgb, Srgba, Hsv,
};

pub struct TemplateFields<'a> {
    pub alpha: u8,
    pub backend: &'a Backend,
    pub palette: &'a Palette,
    pub colorspace: &'a ColorSpace,
    pub image_path: &'a str,
    pub colors: &'a Colors,
}

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
        new_string_template::template::Template::new(file_content)
            // this regex is even better than pywal, doesn't match new lines :3
            // <https://regex101.com/r/AgVXKJ/1>
            .with_regex(&regex::Regex::new(r"\{(\S+?)\}").expect("correct tested regex"))
            .render(&values.to_hash())
            .map_err(|err| format!("Error while rendering '{filename}': {err}"))?
    };

    // map io::Errors into a writeable one (String) ((maybe this is how anyhow werks?))
    std::fs::write(target_path, rendered)
        .map_err(|err| format!("Error while writting to {}: {err}", target_path.display()))
}

/// Writes `template`s into `target`s. Given the many possibilities of I/O errors, template errors,
/// user typos, etc. Most errors are reported to stderr, and ignored to `continue` with the other
/// entries.
pub fn write_template(config_dir: &Path, templates_header: &HashMap<String, Fields>, values: &TemplateFields, quiet: bool) -> Result<()> {

    let mut jinjaenv = jinja_env();
    //XXX loader makes avaliable the (easy) use of `import` and such
    jinjaenv.set_loader(minijinja::path_loader(config_dir));


    // iterate over contents and pass it as an `&String` (which is casted to &str), apply the
    // template and write the templated(?) file to entry.path

    for (name, fields) in templates_header {
        // facilitates strings printing
        let name = name.bold();
        let target = &fields.target.italic();
        let warn = "W".red();
        let warn = warn.bold();

        //root path for the template file
        let path = config_dir.join(&fields.template);

        //root path for the target file (requires interpret `~` for home)
        //XXX on `shellexpand`, think about using `::full()` to support env vars. Seems a bit sketchy/sus
        let env = shellexpand::tilde(&fields.target);
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

fn parse_srgb(s: &str) -> Result<Srgb<u8>, minijinja::Error> {
    Srgb::<u8>::from_str(s)
        .map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, format!("{e}")))
}

fn parse_srgba(s: &str) -> Result<Srgba<u8>, minijinja::Error> {
    Srgba::<u8>::from_str(s)
        .map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, format!("{e}")))
}

pub fn jinja_env<'a>() -> Environment<'a> {
        use minijinja::Error;
        let mut env = Environment::new();
        env.set_keep_trailing_newline(true); // keep the template file intact

        /*filters*/

        // These filters don't require special handling,
        // since they will ignore and don't use alpha whatsoever
        jinjafn!(env, rgb);
        jinjafn!(env, xrgb);
        jinjafn!(env, red);
        jinjafn!(env, green);
        jinjafn!(env, blue);

        /// Blending for usual RRGGBB and RRGGBBAA
        //TODO make this less ugly "but, it werks"
        fn blend(a: String, b: String) -> Result<String, Error> {
            let rgb = parse_srgb(&a);
            let rgba = parse_srgba(&a);

            let rgb1 = parse_srgb(&b);
            let rgba1 = parse_srgba(&b);

            let ret: String = match rgb {
                Ok(o) => {
                    match rgb1 {
                        Ok(o1) => {
                            // SHOULD BE RRGGBB
                            let new = crate::colors::blend(o.into_format(), o1.into_format());
                            let (r, g, b) = new.into_format::<u8>().into_components();
                            format!("#{r:02X}{g:02X}{b:02X}")
                        },
                        Err(_) => {
                            match rgba1 {
                                Ok(o1a) => {
                                    // final output SHOULD BE RRGGBBAA
                                    let new = crate::colors::blend_alpha(o.into_format().into(), o1a.into_format());
                                    let (r, g, b, a) = new.into_format::<u8, u8>().into_components();
                                    format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
                                },
                                Err(_) => {
                                    return Err(minijinja::Error::new(
                                            minijinja::ErrorKind::InvalidOperation,
                                            format!("String '{b}' is not either a hex rgb nor hexa rgba."))
                                    )
                                }
                            }
                        },
                    }
                },
                Err(_) => {
                    match rgba {
                        Ok(oa) => {
                            match rgb1 {
                                Ok(o1) => {
                                    // SHOULD BE RRGGBB
                                    let new = crate::colors::blend((*oa).into_format::<f32>().into(), o1.into_format());
                                    let (r, g, b) = new.into_format::<u8>().into_components();
                                    format!("#{r:02X}{g:02X}{b:02X}")
                                },
                                Err(_) => {
                                    match rgba1 {
                                        Ok(o1a) => {
                                            // final output SHOULD BE RRGGBBAA
                                            let new = crate::colors::blend_alpha(oa.into_format::<f32, f32>().into(), o1a.into_format());
                                            let (r, g, b, a) = new.into_format::<u8, u8>().into_components();
                                            format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
                                        },
                                        Err(_) => {
                                            return Err(minijinja::Error::new(
                                                    minijinja::ErrorKind::InvalidOperation,
                                                    format!("String '{b}' is not either a hex rgb nor hexa rgba."))
                                            )
                                        }
                                    }
                                },
                            }
                        },
                        Err(_) => {
                            return Err(minijinja::Error::new(
                                minijinja::ErrorKind::InvalidOperation,
                                format!("String '{a}' is not either a hex rgb nor hexa rgba."))
                            )
                        },
                    }
                }
            };

            Ok(ret)
        }
        env.add_filter("blend", blend);

        /// Complementary for usual RRGGBB and RRGGBBAA
        fn complementary(s: String) -> Result<String, Error> {
            use crate::colors::Compl;
            let rgb = parse_srgb(&s);
            let rgba = parse_srgba(&s);

            let ret: String = match rgb {
                Ok(o) => {
                    let o: Srgb<f32> = o.into_format();
                    let (r, g, b) = o.complementary().into_format::<u8>().into_components();
                    format!("#{r:02X}{g:02X}{b:02X}")
                },
                Err(_) => {
                    match rgba {
                        Ok(o) => {
                            let o: Srgba<f32> = o.into_format();
                            let (r, g, b, a) = o.complementary().into_format::<u8, u8>().into_components();
                            format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
                        },
                        Err(_) => {
                            return Err(minijinja::Error::new(
                                minijinja::ErrorKind::InvalidOperation,
                                format!("String '{s}' is not either a hex rgb nor hexa rgba."))
                            )
                        },
                    }
                }
            };

            Ok(ret)
        }
        env.add_filter("complementary", complementary);

        /// Saturate function that accepts a RRGGBB or RRGGBBAA
        fn saturate(s: String, arg: f32) -> Result<String, Error> {
            let rgb = parse_srgb(&s);
            let rgba = parse_srgba(&s);

            let ret: String = match rgb {
                Ok(o) => {
                    let o: Hsv = o.into_format::<f32>().into_color();
                    let o: Srgb = o.saturate(arg).into_color();
                    let (r, g, b) = o.into_format::<u8>().into_components();
                    format!("#{r:02X}{g:02X}{b:02X}")
                },
                Err(_) => {
                    match rgba {
                        Ok(o) => {
                            let o: Hsv = o.into_format::<f32, f32>().into_color();
                            let o: Srgba = o.saturate(arg).into_color();
                            let (r, g, b, a) = o.into_format::<u8, u8>().into_components();
                            format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
                        },
                        Err(_) => {
                            return Err(minijinja::Error::new(
                                minijinja::ErrorKind::InvalidOperation,
                                format!("String '{s}' is not either a hex rgb nor hexa rgba."))
                            )
                        },
                    }
                }
            };

            Ok(ret)
        }
        env.add_filter("saturate", saturate);

        /// Darken for usual RRGGBB and RRGGBBAA
        fn darken(s: String, arg: f32) -> Result<String, Error> {
            let rgb = parse_srgb(&s);
            let rgba = parse_srgba(&s);

            let ret: String = match rgb {
                Ok(o) => {
                    let o: Srgb<f32> = o.into_format();
                    let (r, g, b) = o.darken(arg).into_format::<u8>().into_components();
                    format!("#{r:02X}{g:02X}{b:02X}")
                },
                Err(_) => {
                    match rgba {
                        Ok(o) => {
                            let o: Srgba<f32> = o.into_format();
                            let (r, g, b, a) = o.darken(arg).into_format::<u8, u8>().into_components();
                            format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
                        },
                        Err(_) => {
                            return Err(minijinja::Error::new(
                                minijinja::ErrorKind::InvalidOperation,
                                format!("String '{s}' is not either a hex rgb nor hexa rgba."))
                            )
                        },
                    }
                }
            };

            Ok(ret)
        }
        env.add_filter("darken", darken);

        /// Lighten with support for RRGGBBAA aka 'hexa' like values.
        fn lighten(s: String, arg: f32) -> Result<String, Error> {
            let rgb = parse_srgb(&s);
            let rgba = parse_srgba(&s);

            let ret: String = match rgb {
                Ok(o) => {
                    let o: Srgb<f32> = o.into_format();
                    let (r, g, b) = o.lighten(arg).into_format::<u8>().into_components();
                    format!("#{r:02X}{g:02X}{b:02X}")
                },
                Err(_) => {
                    match rgba {
                        Ok(o) => {
                            let o: Srgba<f32> = o.into_format();
                            let (r, g, b, a) = o.lighten(arg).into_format::<u8, u8>().into_components();
                            format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
                        },
                        Err(_) => {
                            return Err(minijinja::Error::new(
                                minijinja::ErrorKind::InvalidOperation,
                                format!("String '{s}' is not either a hex rgb nor hexa rgba."))
                            )
                        },
                    }
                }
            };

            Ok(ret)
        }
        env.add_filter("lighten", lighten);

        /// Strips leading '#' no matter what it is.
        fn strip(hex: String) -> String {
            hex
                .strip_prefix('#')
                .unwrap_or(&hex).to_string()
        }
        env.add_filter("strip", strip);

        /// converts alpha value into a hexadecimal one.
        fn hexa_for_alpha(input: usize) -> Result<String, minijinja::Error> {
            alpha_hexa(input)
                .map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e))
        }
        env.add_filter("alpha_hexa", hexa_for_alpha);

        use std::path::PathBuf;

        /// converts alpha value into a hexadecimal one.
        fn basename(p: ViaDeserialize<PathBuf>) -> Result<String, minijinja::Error> {
            let name = p.file_name();
            match name {
                None => Err(minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, "Cannot get basename")),
                Some(s) => Ok(s.to_string_lossy().to_string()),
            }
        }
        env.add_filter("basename", basename);

        env
}

fn jinja_update_alpha(env: &mut Environment, alpha: u8) {
    env.remove_filter("hexa");
    let hexa = move |value: ViaDeserialize<Myrgb>| -> String {
        let a = alpha_hexa(alpha as usize).expect("number from 0..=100 validated by clap");
        Myrgb::hexa(&value, &a)
    };
    env.add_filter("hexa", hexa);
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

/// hash values
impl TemplateFields<'_> {
pub fn to_hash<'a>(&self) -> HashMap<&'a str, String> {
    let mut map = HashMap::new();
    let alpha = self.alpha;
    let col = self.colors;
    let alpha_hex = alpha_hexa(alpha as usize).expect("CANNOT OVERFLOW, validation with clap 0..=100");
    let alpha_dec = f32::from(alpha) / 100.0;

    //XXX instead of multiple `.method()` maybe using enums and match with a single method

    let vals = include!(concat!(env!("OUT_DIR"), "/template_vals.rs"));

    // has to follow the build.rs order (leaving alpha stuff at last)
    //let funcs = [ "", ".rgb", ".rgba", ".xrgba", ".strip", ".red", ".green", ".blue", ];
    let funcs = [ Myrgb::rgb, Myrgb::strip, Myrgb::red, Myrgb::green, Myrgb::blue, ];

    //full path to the image
    map.insert("wallpaper", self.image_path.into());
    map.insert("alpha", alpha.to_string());
    map.insert("alpha_dec", format!("{alpha_dec:.2}"));
    map.insert("alpha_hex", alpha_hex.clone());

    // Include backend, colorspace and filter (palette)
    map.insert("backend", self.backend.to_string());
    map.insert("colorspace", self.colorspace.to_string());
    map.insert("palette", self.palette.to_string());

    let mut count = 1;

    // 16 colors + 3 (bg, fg and cursor) - 1 (bc is 0 index)
    let len = (16 + 3 - 1) * count;
    // normal output `#EEEEEE`
    for (i, v) in col.into_iter().enumerate() { map.insert(vals[len + i], v.to_string()); }
    map.insert("cursor", col.foreground.to_string());
    count += 1;

    //.rgb output `235,235,235`
    //.strip output `EEEEEE`
    //.red output `235`
    //.green output `235`
    //.blue output `235`
    for j in funcs {
        let len = (16 + 3 - 1) * count;
        for (i, v) in col.into_iter().enumerate() { map.insert(vals[len + i], j(&v)); }
        map.insert("cursor", j(&col.foreground));
        count += 1;
    }

    //.rgba output `235,235,235,1.0`
    let len = (16 + 3 - 1) * count;
    for (i, v) in col.into_iter().enumerate() { map.insert(vals[len + i], v.rgba(alpha_dec)); }
    map.insert("cursor", col.foreground.rgba(alpha_dec));
    count += 1;


    //.xrgba output `ee/ee/ee/ff`
    let len = (16 + 3 - 1) * count;
    for (i, v) in col.into_iter().enumerate() { map.insert(vals[len + i], v.xrgba(&alpha_hex)); }
    map.insert("cursor", col.foreground.xrgba(&alpha_hex));
    //count += 1;

    map
}
}
