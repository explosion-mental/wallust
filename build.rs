
fn main() {
    #[cfg(all(feature = "themes", feature = "buildgen"))]
    themes();
}

#[cfg(all(feature = "themes", feature = "buildgen"))]
/// This adds "random" to the COLS_KEY array such that it can be used as a clap constraint.
/// This is a "workaround" only while making assets. Shell completions benefit from this since clap
/// completions can put all the strings in the array into the completions itself.
fn themes() {
    use wallust_themes::COLS_KEY;

    println!("cargo:rerun-if-changed=build.rs");

    let out = std::env::var_os("OUT_DIR").unwrap();
    let out = std::path::Path::new(&out);

    let mut val = COLS_KEY.to_vec();
    val.push("random");

    let mut s = String::new();
    s.push('[');
    for i in val {
        s.push('"');
        s.push_str(i);
        s.push('"');
        s.push(',');
    }
    s.push(']');

    std::fs::write(out.join("args.rs"), &s).unwrap();
}
