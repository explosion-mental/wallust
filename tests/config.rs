#![cfg(feature = "buildgen")]
/// testing for the validity of the config file is done in `args.rs`
/// a current hack to avoid build.rs hell (requires dividing types into a crate itself or include! hacks)
/// XXX this seems so useful, maybe elaborate this idea into it's own crate.
#[test]
fn main() {
    use std::io::Write;

    //use wallust::config::Config;
    use wallust::backends::Backend;
    use wallust::colorspaces::ColorSpace;
    use wallust::colorspaces::Generate;
    use wallust::palettes::Palette;
    use word_iter::Words;

    //use owo_colors::AnsiColors;
    //use documented::{Documented, DocumentedFields};
    //use documented::DocumentedFields;
    use strum::IntoEnumIterator;

    let v = clap::crate_version!().chars().collect::<Vec<_>>();
    let mut version = String::new();
    let mut num = 0;
    for i in v {
        if i == '.' { num += 1; } // count dots
        if num == 2 { break; }    // only allow Major.Minor
        version.push(i);
    }
    //let version = format!("{M}.{m}", M = v[0], m = v[2]); // crop patch and ignore `-dev`

    // default values
    let def_backend    = Backend::default().to_string().to_ascii_lowercase();
    let def_colorspace = ColorSpace::default().to_string().to_ascii_lowercase();
    let def_palette    = Palette::default().to_string().to_ascii_lowercase();
    let def_threshold  = "20";
    let def_gen        = Generate::default().to_string().to_ascii_lowercase();

    const MAX: usize = 90;
    // const SP: usize = "#      ".len();

    fn ul_comment<T>() -> String
        where T: documented::DocumentedFields + std::fmt::Display + IntoEnumIterator
    {
        // get the padding
        let mut largest = 0;
        for i in T::iter() {
            let i = i.to_string();
            if i.len() > largest {
                largest = i.len();
            }
        }

        // comment
        let mut whole = String::new();
        println!("{:?}", T::iter().map(|x| x.to_string()).collect::<Vec<String>>());
        for i in T::iter() {
            let name = i.to_string();

            // pad the name
            let mut start = format!("{}", name.to_ascii_lowercase());
            if start.len() < largest {
                start.push_str(&" ".repeat(largest - start.len()));
            }

            let init = format!("#  * {start} - ");

            let ind = format!("#    {}    ", " ".repeat(largest));

            let text = T::get_field_docs(name).unwrap();

            // start wrapping the comment
            let mut wraped = vec![String::new()];
            wraped[0].push_str(&init);

            let mut i = 0;
            //let mut len = wraped[i].len();
            //let mut len = 0;

            // iter into words.
            for w in text.words() {
                //len = wraped[i].len() + w.len();
                if (wraped[i].len() + w.len()) > MAX {
                    wraped[i] = wraped[i].trim_end().into();
                    wraped.push("".into());
                    i += 1;
                    wraped[i].push('\n');
                    wraped[i].push_str(&ind);
                    wraped[i].push_str(w);
                    wraped[i].push(' ');
                } else {
                    wraped[i].push_str(w);
                    wraped[i].push(' ');
                }
            }
            // println!("{wraped:#?}");

            let ret = wraped.join("");
            let ret = ret.trim_end();


            // println!("RET = {ret}");

            whole.push_str(&ret);
            whole.push_str("\n");
        }
        whole.trim_end().into()
    }

    let backends    = ul_comment::<Backend>();
    let colorspaces = ul_comment::<ColorSpace>();
    let palettes    = ul_comment::<Palette>();

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

# NOTE: All palettes will fill 16 colors (from color0 to color15), 16 color
#       variations are the 'ilusion' of more colors by opaquing color1 to color5.
# Use the most prominent colors in a way that makes sense, a scheme:
{palettes}
palette = \"{def_palette}\"

# This field chooses a method to use when the gathered colors aren't enough:
#  * interpolation - (default) Tries to pick two colors and built gradients over them
#  * complementary - Uses the complementary colors of two colors, or more (if needed), colors.
#generation = \"{def_gen}\"

# Ensures a \"readable contrast\" (OPTIONAL, disabled by default)
# Should only be enabled when you notice an unreadable contrast frequently happening
# with your images. The reference color for the contrast is the background color.
#check_contrast = true

# Color saturation, between [1% and 100%] (OPTIONAL, disabled by default)
# usually something higher than 50 increases the saturation and below
# decreases it (on a scheme with strong and vivid colors)
#saturation = 35

# Alpha value for templating, by default 100 (no other use whatsoever)
#alpha = 100
");

let raw_part =
r#"
[templates]
# template: A relative path that points to a file where wallust.toml is located, usually at `~/.config/wallust/`.
# target: Absolute path in which to place a file with generated templated values.
# If one is a directory, the other one NEEDS to also be one.
# NOTE: prefer '' over "" for paths, avoids escaping.
# refer to the man page for more info: `man wallust.5`
# Example:
#zathura = { template = 'zathura', target = '~/.config/zathura/zathurarc' }
"#;

    let template = template + raw_part;

    std::fs::File::create("wallust.toml").unwrap()
        .write_all(template.as_bytes()).unwrap()
}
