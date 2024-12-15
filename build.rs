fn main() {
    #[cfg(all(feature = "themes", feature = "buildgen"))]
    // This adds "random" to the COLS_KEY array such that it can be used as a clap constraint.
    // This is a "workaround" only while making assets. Shell completions benefit from this since clap
    // completions can put all the strings in the array into the completions itself.
    {
        use wallust_themes::COLS_KEY;
        println!("cargo:rerun-if-changed=build.rs");

        let mut val = COLS_KEY.to_vec();
        val.push("random");
        val.push("list");

        std::fs::write(outdir().join("args.rs"), to_literal_vec(val)).unwrap();
    }

    //version (sha date)
    let s = format!(r#""{} {}""#, clap::crate_version!(), version());
    std::fs::write(outdir().join("version.rs"), s).unwrap();

    //template const values
    std::fs::write(outdir().join("template_vals.rs"), to_literal_vec(template_vals())).unwrap();
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


fn outdir() -> std::path::PathBuf {
    let out = std::env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out).into()
}

use std::fmt::Display;

/// converts a vec to a literal one that can be assign at comp time
fn to_literal_vec<T: Display>(v: Vec<T>) -> String {
    let mut val: Vec<_> = v.iter().map(|i| format!(r#""{i}","#)).collect(); //"string",
    val.insert(0, "[".to_string()); //start of array
    val.push("]".to_string());
    val.join("")
}

fn template_vals() -> Vec<String> {
    let mut list = vec![];
    for i in 0..15 { list.push(format!("color{i}")); }
    list.push("background".into());
    list.push("foreground".into());
    list.push("cursor".into());

    let funcs = [ "", ".rgb", ".strip", ".red", ".green", ".blue", ".rgba", "xrgba" ];

    let mut ret = vec![];

    for j in funcs {
        for i in &list {
            ret.push(format!("{i}{j}"));
        }
    }

    ret
}
