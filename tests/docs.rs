/// testing for the validity of the config file is done in `args.rs`
/// a current hack to avoid build.rs hell (requires dividing types into a crate itself or include! hacks)
/// XXX this seems so useful, maybe elaborate this idea into it's own crate.
#[cfg(all(feature = "doc", feature = "iter"))]
#[test]
fn main() {
    use std::io::Write;
    //use wallust::config::Config;
    use wallust::backends::Backend;
    use wallust::colorspaces::ColorSpace as ColorSpaces;
    // use wallust::colorspaces::FallbackGenerator as Generate;
    use wallust::palettes::Palette as Filters;
    use strum::IntoEnumIterator;

    fn ul_comment<T: documented::DocumentedFields + std::fmt::Display + IntoEnumIterator>() -> String {
        let mut ret = String::new();

        for i in T::iter() {
            let name = i.to_string();
            let desc = T::get_field_docs(name.clone()).unwrap();

            //join multiple lines into a one liner
            let desc = desc.split('\n').collect::<Vec<&str>>();
            let desc = desc.join(" ");

            ret.push_str(&format!("**{name}** | {desc}"));
            ret.push('\n');
        }
        ret
    }

    let backends    = ul_comment::<Backend>();
    let colorspaces = ul_comment::<ColorSpaces>();
    let filters     = ul_comment::<Filters>();

    let backend = format!("\
| Backends  | Description |
|-----------|-------------|
{backends}");

    let colorspace = format!("\
| Color Space | Description |
|-------------|-------------|
{colorspaces}");

    let palette = format!("\
| Palette | Description |
|---------|-------------|
{filters}");

    std::fs::File::create("docs/parameters/backend-table.md").unwrap()
        .write_all(backend.as_bytes()).unwrap();

    std::fs::File::create("docs/parameters/colorspace-table.md").unwrap()
        .write_all(colorspace.as_bytes()).unwrap();

    std::fs::File::create("docs/parameters/palette-table.md").unwrap()
        .write_all(palette.as_bytes()).unwrap();
}
