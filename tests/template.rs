#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use wallust::backends::Backend;
use wallust::colors::Colors;
//use wallust::colors::HexConversion;
use wallust::colors::Myrgb;
use wallust::palettes::Palette;
use wallust::template::jinja_env;
use wallust::template::TemplateFields;
//use wallust::colors;
use palette::Srgb;

//TODO add tests for every KEY combination

/// Sample colors in use
fn mycols() -> Colors {
    Colors {
        background: Myrgb(Srgb::new(238_u8, 238, 238).into_format()), //#EEEEEE
        foreground: Myrgb(Srgb::new(221_u8, 221, 221).into_format()), //#DDDDDD
        color0 : Myrgb(Srgb::new(0 , 0_u8, 0).into_format()), //# 00 00 00
        color1 : Myrgb(Srgb::new(1 , 0_u8, 0).into_format()), //# 01 00 00
        color2 : Myrgb(Srgb::new(2 , 0_u8, 0).into_format()),
        color3 : Myrgb(Srgb::new(3 , 0_u8, 0).into_format()),
        color4 : Myrgb(Srgb::new(4 , 0_u8, 0).into_format()),
        color5 : Myrgb(Srgb::new(5 , 0_u8, 0).into_format()),
        color6 : Myrgb(Srgb::new(6 , 0_u8, 0).into_format()),
        color7 : Myrgb(Srgb::new(7 , 0_u8, 0).into_format()),
        color8 : Myrgb(Srgb::new(8 , 0_u8, 0).into_format()),
        color9 : Myrgb(Srgb::new(9 , 0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(10, 0_u8, 0).into_format()), //# 0A 00 00
        color11: Myrgb(Srgb::new(11, 0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(12, 0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(13, 0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(14, 0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(15, 0_u8, 0).into_format()), //# 0F 00 00
    }
}

const wall_str: &str = "/home";


/// set up minijinja for rendering something
fn jinja(content: &str) -> String {
    let Tfields: &TemplateFields = &TemplateFields {
        alpha: 100,
        backend: &Backend::Thumb,
        palette: &Palette::Dark,
        colorspace: &wallust::colorspaces::ColorSpace::Lab,
        image_path: wall_str,
        colors: &mycols(),
    };
    let v = minijinja::Value::from(Tfields);
    jinja_env(Tfields.alpha).render_named_str("sample", content, v).unwrap()
}

/// Test template variables: `{color0}` - `{color15}`, bg, fg, cursor and wallpaper
#[test]
fn colors() {
    let expected = [
        "#000000",
        "#010000",
        "#020000",
        "#030000",
        "#040000",
        "#050000",
        "#060000",
        "#070000",
        "#080000",
        "#090000",
        "#0A0000",
        "#0B0000",
        "#0C0000",
        "#0D0000",
        "#0E0000",
        "#0F0000",
    ];
    for i in 0..16 {
        let mut sample = String::from("{{color");
        sample.push_str(&i.to_string());
        sample.push_str("}}");

        let result = jinja(&sample);
        assert_eq!(expected[i], result);
    }
}

#[test]
fn filters() {
    let COLS = mycols();
    let expected = [
        COLS.color0,
        COLS.color1,
        COLS.color2,
        COLS.color3,
        COLS.color4,
        COLS.color5,
        COLS.color6,
        COLS.color7,
        COLS.color8,
        COLS.color9,
        COLS.color10,
        COLS.color11,
        COLS.color12,
        COLS.color13,
        COLS.color14,
        COLS.color15,
    ];
    for i in 0..16 {
        let c = format!("color{i}");
        let br1 = "{{";
        let br2 = "}}";

        let strip = format!("{br1}{c} | strip {br2}");
        let strip = jinja(&strip);
        assert_eq!(expected[i].strip(), strip);

        let rgb = format!("{br1}{c} | rgb {br2}");
        let rgb = jinja(&rgb);
        assert_eq!(expected[i].rgb(), rgb);
    }
}

/// Like the above but with pywal syntax
#[test]
fn pywal() {
    let Tfields: &TemplateFields = &TemplateFields {
        alpha: 100,
        backend: &Backend::Thumb,
        palette: &Palette::Dark,
        colorspace: &wallust::colorspaces::ColorSpace::Lab,
        image_path: wall_str,
        colors: &mycols(),
    };

    let sample =
r#"
# Special
wallpaper="{wallpaper}"
background='{background}'
foreground='{foreground}'
cursor='{cursor}'

# Colors
color0='{color0}'
color1='{color1}'
color2='{color2}'
color3='{color3}'
color4='{color4}'
color5='{color5}'
color6='{color6}'
color7='{color7}'
color8='{color8}'
color9='{color9}'
color10='{color10}'
color11='{color11}'
color12='{color12}'
color13='{color13}'
color14='{color14}'
color15='{color15}'
"#;

    let expected_output =
r#"
# Special
wallpaper="/home"
background='#EEEEEE'
foreground='#DDDDDD'
cursor='#DDDDDD'

# Colors
color0='#000000'
color1='#010000'
color2='#020000'
color3='#030000'
color4='#040000'
color5='#050000'
color6='#060000'
color7='#070000'
color8='#080000'
color9='#090000'
color10='#0A0000'
color11='#0B0000'
color12='#0C0000'
color13='#0D0000'
color14='#0E0000'
color15='#0F0000'
"#;
    let result = new_string_template::template::Template::new(sample)
        // this regex is even better than pywal, doesn't match new lines :3
        // <https://regex101.com/r/AgVXKJ/1>
        .with_regex(&regex::Regex::new(r"\{(\S+?)\}").expect("correct tested regex"))
        .render(&Tfields.to_hash())
        .unwrap()
        ;

    assert_eq!(expected_output, result);
}
