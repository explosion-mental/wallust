#![cfg(feature = "schema")]
/// testing for the validity of the config file is done in `args.rs`
/// a current hack to avoid build.rs hell (requires dividing types into a crate itself or include! hacks)
/// XXX this seems so useful, maybe elaborate this idea into it's own crate.
#[test]
fn jsonschema() {
    use schemars::schema_for;
    use std::io::Write;
    use wallust::config::PrettyConfig;

    let schema = schema_for!(PrettyConfig);
    let content = serde_json::to_string_pretty(&schema).unwrap();

    std::fs::File::create("schema.json").unwrap()
        .write_all(content.as_bytes()).unwrap();
}
