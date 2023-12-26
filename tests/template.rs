use std::fs::File;
use std::io::Write;
use std::path::Path;

use wallust::colors::Colors;
//use wallust::colors::HexConversion;
use wallust::colors::Myrgb;
use wallust::template;
use wallust::config;
//use wallust::colors;

//TODO add tests for every KEY combination

const COLS: &Colors = &Colors {
        background: Myrgb(238, 238, 238), //#EEEEEE
        foreground: Myrgb(221, 221, 221), //#DDDDDD
        color0 : Myrgb(0 , 0, 0), //# 00 00 00
        color1 : Myrgb(1 , 0, 0), //# 01 00 00
        color2 : Myrgb(2 , 0, 0),
        color3 : Myrgb(3 , 0, 0),
        color4 : Myrgb(4 , 0, 0),
        color5 : Myrgb(5 , 0, 0),
        color6 : Myrgb(6 , 0, 0),
        color7 : Myrgb(7 , 0, 0),
        color8 : Myrgb(8 , 0, 0),
        color9 : Myrgb(9 , 0, 0),
        color10: Myrgb(10, 0, 0), //# 0A 00 00
        color11: Myrgb(11, 0, 0),
        color12: Myrgb(12, 0, 0),
        color13: Myrgb(13, 0, 0),
        color14: Myrgb(14, 0, 0),
        color15: Myrgb(15, 0, 0), //# 0F 00 00
    };


/// Test template variables: `{color0}` - `{color15}`, bg, fg, cursor and wallpaper
#[test]
fn variables() {
    let template_sample =
"\
# Special
wallpaper=\"{wallpaper}\"
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
";

    let target_sample =
"\
# Special
wallpaper=\"\"
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
";

    let tmpdir = tempfile::tempdir().expect("init new temporal named pipe");

    // file in which the template variables are
    let template_path = tmpdir.path().join("colors-template.sh");
    let mut template = File::create(&template_path).expect("should created a tmp file");
    write!(template, "{template_sample}").expect("should write to tmp correctly");

    // file in which, after templating, it's gonna be writeable
    let target_path = tmpdir.path().join("colors.sh");
    let mut _target = File::create(&target_path).expect("should created a tmp file");

    // usual config
    let c = config::Config {
        check_contrast: None,
        dir: tmpdir.path().into(),
        entry: Some(vec![config::Entries {
            template: template_path.display().to_string(),
            target: target_path.display().to_string(),
            new_engine: Some(false),
        }]),
        ..config::Config::default()
    };

    let e = c.entry.as_ref().unwrap();

    template::write_template(&c, Path::new(""), &e, COLS, true).expect("should parse correctly");

    //store templated string
    let target_content = std::fs::read_to_string(&e[0].target).expect("TARGET CONTENT");

    // templated file should be the same as expected in the target_sample
    assert_eq!(target_content, target_sample);

    // `rm`ing the temporal dir will also close the files inside it
    tmpdir.close().expect("temporal directory should close successfully");
}

/// Like the above but with the new engine
#[test]
fn variables_new_engine() {
    let template_sample =
r#"
# Special
wallpaper="{{wallpaper}}"
background='{{background}}'
foreground='{{foreground}}'
cursor='{{cursor}}'

# Colors
color0='{{color0}}'
color1='{{color1}}'
color2='{{color2}}'
color3='{{color3}}'
color4='{{color4}}'
color5='{{color5}}'
color6='{{color6}}'
color7='{{color7}}'
color8='{{color8}}'
color9='{{color9}}'
color10='{{color10}}'
color11='{{color11}}'
color12='{{color12}}'
color13='{{color13}}'
color14='{{color14}}'
color15='{{color15}}'
"#;

    let target_sample =
r#"
# Special
wallpaper=""
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

    let tmpdir = tempfile::tempdir().expect("init new temporal named pipe");

    // file in which the template variables are
    let template_path = tmpdir.path().join("colors-template.sh");
    let mut template = File::create(&template_path).expect("should created a tmp file");
    write!(template, "{template_sample}").expect("should write to tmp correctly");

    // file in which, after templating, it's gonna be writeable
    let target_path = tmpdir.path().join("colors.sh");
    let mut _target = File::create(&target_path).expect("should created a tmp file");

    // usual config
    let c = config::Config {
        check_contrast: None,
        dir: tmpdir.path().into(),
        entry: Some(vec![config::Entries {
            template: template_path.display().to_string(),
            target: target_path.display().to_string(),
            new_engine: Some(true),
        }]),
        ..config::Config::default()
    };

    let e = c.entry.as_ref().unwrap();

    template::write_template(&c, Path::new(""), &e, COLS, true).expect("should parse correctly");

    //store templated string
    let target_content = std::fs::read_to_string(&e[0].target).expect("TARGET CONTENT");

    // templated file should be the same as expected in the target_sample
    assert_eq!(target_content, target_sample);

    // `rm`ing the temporal dir will also close the files inside it
    tmpdir.close().expect("temporal directory should close successfully");
}
