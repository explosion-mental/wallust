//! Pywal Template testing
//! Here we use the templates from the pywal repo, and ensure they template correctly.
//! https://github.com/dylanaraps/pywal/tree/master/pywal/templates
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use wallust::backends::Backend;
use wallust::palettes::Palette;
use wallust::template::TemplateFields;
use wallust::template::pywal;

mod template;
use template::mycols;
use template::wall_str;

macro_rules! waltest {
    ($func_name:ident, $template:expr, $expected:expr) => {
        #[test]
        fn $func_name() {
            let Tfields: &TemplateFields = &TemplateFields {
                alpha: 100,
                backend: &Backend::Thumb,
                palette: &Palette::Dark,
                colorspace: &wallust::colorspaces::ColorSpace::Lab,
                image_path: wall_str,
                colors: &mycols(),
            };

            let template = include_str!($template);
            let expected = include_str!($expected);
            let result = pywal::render(template, Tfields).unwrap();
            assert_eq!(expected, result);
        }
    };
}

waltest!(wal_colors, "./pywal-templates/colors", "./pywal-templates-result/colors");
waltest!(wal_haskell, "./pywal-templates/colors.hs", "./pywal-templates-result/colors.hs");
waltest!(wal_shell, "./pywal-templates/colors.sh", "./pywal-templates-result/colors.sh");
waltest!(wal_css, "./pywal-templates/colors.css", "./pywal-templates-result/colors.css");
waltest!(wal_yml, "./pywal-templates/colors.yml", "./pywal-templates-result/colors.yml");
waltest!(wal_sway, "./pywal-templates/colors-sway", "./pywal-templates-result/colors-sway");
waltest!(wal_json, "./pywal-templates/colors.json", "./pywal-templates-result/colors.json");
waltest!(wal_scss, "./pywal-templates/colors.scss", "./pywal-templates-result/colors.scss");
waltest!(wal_styl, "./pywal-templates/colors.styl", "./pywal-templates-result/colors.styl");
waltest!(wal_oomox, "./pywal-templates/colors-oomox", "./pywal-templates-result/colors-oomox");
waltest!(wal_tty, "./pywal-templates/colors-tty.sh", "./pywal-templates-result/colors-tty.sh");
waltest!(wal_nqq, "./pywal-templates/colors-nqq.css", "./pywal-templates-result/colors-nqq.css");
waltest!(wal_vim, "./pywal-templates/colors-wal.vim", "./pywal-templates-result/colors-wal.vim");
waltest!(wal_st, "./pywal-templates/colors-wal-st.h", "./pywal-templates-result/colors-wal-st.h");
waltest!(wal_putty, "./pywal-templates/colors-putty.reg", "./pywal-templates-result/colors-putty.reg");
waltest!(wal_themer, "./pywal-templates/colors-themer.js", "./pywal-templates-result/colors-themer.js");
waltest!(wal_waybar, "./pywal-templates/colors-waybar.css", "./pywal-templates-result/colors-waybar.css");
waltest!(wal_vscode, "./pywal-templates/colors-vscode.json", "./pywal-templates-result/colors-vscode.json");
waltest!(wal_dmenu, "./pywal-templates/colors-wal-dmenu.h", "./pywal-templates-result/colors-wal-dmenu.h");
waltest!(wal_konsole, "./pywal-templates/colors-konsole.colorscheme", "./pywal-templates-result/colors-konsole.colorscheme");
waltest!(wal_speedcrunch, "./pywal-templates/colors-speedcrunch.json", "./pywal-templates-result/colors-speedcrunch.json");
waltest!(wal_rofi_light, "./pywal-templates/colors-rofi-light.rasi", "./pywal-templates-result/colors-rofi-light.rasi");
waltest!(wal_rofi_dark, "./pywal-templates/colors-rofi-dark.rasi", "./pywal-templates-result/colors-rofi-dark.rasi");
waltest!(wal_tabbed, "./pywal-templates/colors-wal-tabbed.h", "./pywal-templates-result/colors-wal-tabbed.h");
waltest!(wal_tilix, "./pywal-templates/colors-tilix.json", "./pywal-templates-result/colors-tilix.json");
waltest!(wal_Xresources, "./pywal-templates/colors.Xresources", "./pywal-templates-result/colors.Xresources");
waltest!(wal_dwm, "./pywal-templates/colors-wal-dwm.h", "./pywal-templates-result/colors-wal-dwm.h");
waltest!(wal_kitty, "./pywal-templates/colors-kitty.conf", "./pywal-templates-result/colors-kitty.conf");

