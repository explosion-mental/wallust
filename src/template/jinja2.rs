//! mini Jinja2 Template Engine
//! This is integrating a subset of jinja2 string formatting engine to wallust templates. I've
//! chosen minijinja because it's simplicity and the powerful parser it could end up (enough for
//! either a "noobie" or an experience ricer).
//! Refs:
//! - https://github.com/mitsuhiko/minijinja/blob/main/COMPATIBILITY.md
use std::str::FromStr;

use crate::colors::Myrgb;
use super::alpha_hexa;
use super::TemplateFields;

use anyhow::Result;
use minijinja::Environment;
use minijinja::context;
use minijinja::value::ViaDeserialize;

use palette::{
    Darken, Lighten, IntoColor, Saturate,
    Srgb, Srgba, Hsv,
};

/// Simple macro to simplify converting methods to jinja filters
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

impl From<&TemplateFields<'_>> for minijinja::Value {
    fn from(values: &TemplateFields<'_>) -> Self {
        let c = &values.colors;
        let alpha_dec = f32::from(values.alpha) / 100.0;
        let alpha_dec = if values.alpha % 10 == 0 { format!("{alpha_dec:.1}") } else { format!("{alpha_dec:.2}") };
        let v = minijinja::Value::from_serialize(c);

        context! {
            ..v,
            ..context! {
                alpha      => values.alpha,
                alpha_dec  => alpha_dec,
                cursor     => c.cursor,
                palette    => values.palette,
                wallpaper  => values.image_path,
                backend    => values.backend,
                colorspace => values.colorspace,
                colors     => c.into_iter().map(|x| x.to_string()).collect::<Vec<String>>(),
            }
        }

    }
}


/// Recommended way to chain errors
/// ref: <https://docs.rs/minijinja/latest/minijinja/struct.Error.html>
pub fn minijinja_err_chain(err: minijinja::Error) -> String {
    let mut err = &err as &dyn std::error::Error;
    let mut s = format!("Could not render template: {err:#}");

    // get to the source, if there are more.
    while let Some(next_err) = err.source() {
        s.push_str(&format!("\nCaused by: {next_err:#}"));
        err = next_err;
    }
    s
}

/// Simple fn for `map_err` to convert a simple error to a minijinja error
fn jinjerr<T: std::error::Error>(err: T) -> minijinja::Error {
    minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, format!("{err}"))
}

fn parse_srgb(s: &str) -> Result<Srgb<u8>, minijinja::Error> {
    Srgb::<u8>::from_str(s).map_err(jinjerr)
}

fn parse_srgba(s: &str) -> Result<Srgba<u8>, minijinja::Error> {
    Srgba::<u8>::from_str(s).map_err(jinjerr)
}

pub fn jinja_env<'a>() -> Environment<'a> {
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
        jinjafn!(env, rgbf);
        jinjafn!(env, redf);
        jinjafn!(env, greenf);
        jinjafn!(env, bluef);

        /// Blending for usual RRGGBB and RRGGBBAA
        //TODO make this less ugly "but, it werks"
        fn blend(a: String, b: String) -> Result<String, minijinja::Error> {
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
        fn complementary(s: String) -> Result<String, minijinja::Error> {
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
        fn saturate(s: String, arg: f32) -> Result<String, minijinja::Error> {
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
        fn darken(s: String, arg: f32) -> Result<String, minijinja::Error> {
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
        fn lighten(s: String, arg: f32) -> Result<String, minijinja::Error> {
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

pub fn jinja_update_alpha(env: &mut Environment, alpha: u8) {
    env.remove_filter("hexa");
    let hexa = move |value: ViaDeserialize<Myrgb>| -> String {
        let a = alpha_hexa(alpha as usize).expect("number from 0..=100 validated by clap");
        Myrgb::hexa(&value, &a)
    };
    env.add_filter("hexa", hexa);
}

