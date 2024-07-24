fn main() {
    #[cfg(all(feature = "themes", feature = "buildgen"))]
    themes();

    let out = std::env::var_os("OUT_DIR").unwrap();
    let ver = std::path::Path::new(&out).join("version.rs");
    let s = format!(r#""{} {}""#, clap::crate_version!(), version());
    std::fs::write(ver, s).unwrap();
}

/// git short sha1 and date stuff, only when wallust is at a unstable version
fn version() -> String {
    use vergen_git2 as vergen;
    let git2 = vergen::Git2Builder::default()
        .describe(true, false, None)
        .commit_date(true)
        .sha(true)
        .build().unwrap();
    vergen::Emitter::default()
        .add_instructions(&git2).unwrap()
        .emit_and_set().unwrap();

    let sha = std::env::var_os("VERGEN_GIT_SHA").unwrap();
    let sha = sha.to_string_lossy();

    let describe = std::env::var_os("VERGEN_GIT_DESCRIBE").unwrap();
    let describe = describe.to_string_lossy();

    let date = std::env::var_os("VERGEN_GIT_COMMIT_DATE").unwrap();
    let date = date.to_string_lossy();

    //XXX while we could just check for the branch, it could be that `dev` is the same as `master`
    if sha == describe { //we are on a released version
        String::new()
    } else { // development version
        format!("({sha} {date})")
    }
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
