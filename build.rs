fn main() {
    #[cfg(feature = "themes")]
    themes();

    //version (sha date)
    let s = format!(r#""{} {}""#, clap::crate_version!(), version());
    std::fs::write(outdir().join("version.rs"), s).unwrap();
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

    let date = std::env::var_os("VERGEN_GIT_COMMIT_DATE").unwrap();
    let date = date.to_string_lossy();

    format!("({sha} {date})")
}

#[cfg(feature = "themes")]
/// This adds "random" to the COLS_KEY array such that it can be used as a clap constraint.
/// This is a "workaround" only while making assets. Shell completions benefit from this since clap
/// completions can put all the strings in the array into the completions itself.
fn themes() {
    use wallust_themes::COLS_KEY;
    println!("cargo:rerun-if-changed=build.rs");

    let mut val = COLS_KEY.to_vec();
    val.push("random");
    val.push("list");
    let mut val: Vec<_> = val.iter().map(|i| format!(r#""{i}","#)).collect(); //"string",
    val.insert(0, "[".to_string()); //start of array
    val.push("]".to_string());

    std::fs::write(outdir().join("args.rs"), val.join("")).unwrap();
}

fn outdir() -> std::path::PathBuf {
    let out = std::env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out).into()
}
