use tempfile::tempdir;
use wallust::cache::Cache;
use wallust::cache::IsCached;
use std::fs::File;
use std::io::Write;


//cat avril-blue-and-blackwhite.jpg_1060213_1986482_1.0.json | sed 's/"/\\"/g' | xclip -i

/// Should pase the sample json cache
#[test]
fn is_cached() {
    let sample = r##"{"cursor":"#FDF6DD","background":"#040100","foreground":"#FDF6DD","color0":"#050201","color1":"#415F58","color2":"#92473A","color3":"#BA4839","color4":"#B4592D","color5":"#B78033","color6":"#BDAE78","color7":"#F5EBC7","color8":"#ABA48B","color9":"#456C64","color10":"#B14C3B","color11":"#E21D03","color12":"#E7601F","color13":"#F4AB45","color14":"#FCE8A0","color15":"#F5EBC7"}"##;

    let tmp_dir = tempdir().unwrap();

    let tmp_back = tmp_dir.path().join("test-backend");
    File::create(&tmp_back).unwrap();

    let tmp_cs = tmp_dir.path().join("test-colorspace");
    File::create(&tmp_cs).unwrap();

    let tmp_palet = tmp_dir.path().join("test-palette");
    let mut tmpfile_palet = File::create(&tmp_palet).unwrap();
    write!(tmpfile_palet, "{sample}").unwrap();

    let c = Cache {
        back: tmp_back,
        cs: tmp_cs,
        palette: tmp_palet,
        ..Cache::default()
    };

    c.read_palette().expect("sample format is OK, shouldn't fail");

    assert!(matches!(c.is_cached_all(), IsCached::BackendnCSnPalette));

    tmp_dir.close().expect("temporal named pipe should close successfully");
}
