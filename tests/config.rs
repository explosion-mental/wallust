/// testing for the validity of the config file is done in `args.rs`
/// a current hack to avoid build.rs hell (requires dividing types into a crate itself or include! hacks)
/// XXX this seems so useful, maybe elaborate this idea into it's own crate.
#[cfg(feature = "makeconfig")]
#[test]
fn main() {
    use std::io::Write;

    //use wallust::config::Config;
    use wallust::backends::Backend;
    use wallust::colorspaces::ColorSpaces;
    use wallust::filters::Filters;

    //use owo_colors::AnsiColors;
    //use documented::{Documented, DocumentedFields};
    //use documented::DocumentedFields;
    use strum::IntoEnumIterator;

    let version = clap::crate_version!();
    let version = &version[0..version.len() - 2]; // crop patch

    // default values
    let def_backend    = Backend::Resized.to_string().to_ascii_lowercase();
    let def_colorspace = ColorSpaces::Lab.to_string().to_ascii_lowercase();
    let def_filter     = Filters::SoftDark16.to_string().to_ascii_lowercase();
    let def_threshold  = "20";

    // const MAX: usize = 90;
    // const SP: usize = "#      ".len();

    fn ul_comment<T>() -> String
        where T: documented::DocumentedFields + std::fmt::Display + IntoEnumIterator
    {
        let mut largest = 0;
        for i in T::iter() {
            let i = i.to_string();
            if i.len() > largest {
                largest = i.len();
            }
        }

        let mut whole = String::new();
        for i in T::iter() {
            let mut start = format!("{}", i.to_string().to_ascii_lowercase());
            if start.len() < largest {
                start.push_str(&" ".repeat(largest - start.len()));
            }
            start = format!("#  * {start} - {}\n", T::get_field_comment(i.to_string()).unwrap());
            // TODO make the text wrap, respecting words (wrap on a whitespace)
            // let mut comment = T::get_field_comment(i.to_string()).unwrap().to_string();
            //let len = comment.len() + start.len();
            // if len > MAX {
            //     let (first, second) = comment.split_at(MAX);
            //     let second = format!("#    {}  {second}", " ".repeat(largest));
            //     comment = format!("{first}\n{second}");
            // }
            // start = format!("#  * {start} - {}\n", comment);
            whole.push_str(&start);
        }
        whole.trim_end().into()
    }

    let backends    = ul_comment::<Backend>();
    let colorspaces = ul_comment::<ColorSpaces>();
    let filters     = ul_comment::<Filters>();

    let template = format!(
"# wallust {version}.*
# -- global space -- #
# values below can be overwritten by command line flags

# How the image is parse, in order to get the colors:
{backends}
backend = \"{def_backend}\"

# What color space to use to produce and select the most prominent colors:
{colorspaces}
color_space = \"{def_colorspace}\"

# Difference between similar colors, used by the colorspace:
#  1          Not perceptible by human eyes.
#  1 - 2      Perceptible through close observation.
#  2 - 10     Perceptible at a glance.
#  11 - 49    Colors are more similar than opposite
#  100        Colors are exact opposite
threshold = {def_threshold}

# NOTE: All filters will fill 16 colors (from color0 to color15), 16 color
#       variations are the 'ilusion' of more colors by opaquing color1 to color5.
# Use the most prominent colors in a way that makes sense, a scheme:
{filters}
filter = \"{def_filter}\"

# Ensures a \"readable contrast\" (OPTIONAL, disabled by default)
# Should only be enables when you notice an unreadable contrast frequently happening
# with your images. The reference color for the contrast is the background color.
#check_contrast = true

# Color saturation, between [1% and 100%] (OPTIONAL, disabled by default)
# usually something higher than 50 increases the saturation and below
# decreases it (on a scheme with strong and vivid colors)
#saturation = 35

# Alpha value for templating, by default 100 (no other use whatsoever)
#alpha = 100

# -- templating -- # (OPTIONAL)
# An `entry` requires two files:
# 1. template: A relative path that points to a file where wallust.toml is located, usually at `~/.config/wallust/`
# 2. target: Absolute path in which to place a file with generated templated values

# This is the most common way of integrating `wallust` generated palette to some program.
# Below a simple example that searches for `config-path/zathurarc` and puts the newly created file to `~/.config/zathura/zathurarc`

# [[entry]]
# template = \"zathurarc\"
# target = \"~/.config/zathura/zathurarc\"
"
);

    std::fs::File::create("wallust.toml").unwrap()
        .write_all(template.as_bytes()).unwrap()
}
