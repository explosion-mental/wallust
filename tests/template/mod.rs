#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use wallust::colors::Colors;
use wallust::colors::Myrgb;
use palette::Srgb;

/// Sample colors in use
pub fn mycols() -> Colors {
    Colors {
        cursor: Myrgb(Srgb::new(221_u8, 221, 221).into_format()), //#DDDDDD
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

pub const wall_str: &str = "/home";
