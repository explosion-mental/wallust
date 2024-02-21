use wallust::cache::Cache;
use std::io::Write;


//cat avril-blue-and-blackwhite.jpg_1060213_1986482_1.0.json | sed 's/"/\\"/g' | xclip -i

/// Should pase the sample json cache
#[test]
fn parse_cache() {
    let sample = r##"{"background":"#040100","foreground":"#FDF6DD","color0":"#050201","color1":"#415F58","color2":"#92473A","color3":"#BA4839","color4":"#B4592D","color5":"#B78033","color6":"#BDAE78","color7":"#F5EBC7","color8":"#ABA48B","color9":"#456C64","color10":"#B14C3B","color11":"#E21D03","color12":"#E7601F","color13":"#F4AB45","color14":"#FCE8A0","color15":"#F5EBC7"}"##;

    let mut tmp = tempfile::NamedTempFile::new().expect("init new temporal named pipe");
    write!(tmp, "{sample}").unwrap();

    let mut c = Cache {
        normal: tmp.path().into(), // no fallback generator
        path: tmp.path().into(),   // path same as normal, `gen` doesn't exist then
        ..Cache::default()
    };

    c.read().expect("sample format is OK, shouldn't fail");

    assert!(c.is_cached());

    tmp.close().expect("temporal named pipe should close successfully");
}
