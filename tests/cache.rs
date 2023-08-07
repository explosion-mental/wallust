use wallust::cache;
use std::io::Write;


//cat avril-blue-and-blackwhite.jpg_1060213_1986482_1.0.json | sed 's/"/\\"/g' | xclip -i

/// Should pase the sample json cache
#[test]
fn parse_cache() {
    let sample = "{\"background\":[5,5,5],\"foreground\":[231,230,231],\"color0\":[5,5,5],\"color1\":[67,47,32],\"color2\":[72,70,71],\"color3\":[115,70,49],\"color4\":[101,101,107],\"color5\":[154,119,96],\"color6\":[141,138,141],\"color7\":[213,211,213],\"color8\":[149,147,149],\"color9\":[90,63,43],\"color10\":[97,94,95],\"color11\":[154,94,66],\"color12\":[135,135,143],\"color13\":[206,159,129],\"color14\":[189,184,188],\"color15\":[170,168,170]}";

    let mut tmp = tempfile::NamedTempFile::new().expect("init new temporal named pipe");
    write!(tmp, "{sample}").unwrap();

    let c = cache::Cache { path: tmp.path().display().to_string() };

    c.read().expect("sample format is OK, shouldn't fail");

    tmp.close().expect("temporal named pipe should close successfully");
}
