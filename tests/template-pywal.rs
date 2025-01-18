#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use wallust::backends::Backend;
use wallust::palettes::Palette;
use wallust::template::TemplateFields;
use wallust::template::pywal;

mod template;
use template::mycols;
use template::wall_str;

///TODO just download all templates from pywal and test them
// https://github.com/dylanaraps/pywal/tree/master/pywal/templates
#[test]
fn pywal_render() {
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
    let result = pywal::render(sample, Tfields).unwrap();

    assert_eq!(expected_output, result);
}
