#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use wallust::backends::Backend;
use wallust::palettes::Palette;
use wallust::template::jinja2::jinja_env;
use wallust::template::TemplateFields;

mod template;
use template::wall_str;
use template::mycols;

//TODO add tests for every KEY combination

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
    jinja_env().render_named_str("sample", content, v).unwrap()
}

/// Test template variables: `{color0}` - `{color15}`, bg, fg, cursor and wallpaper
#[test]
fn jinja_colors() {
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
fn jinja_filters() {
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
