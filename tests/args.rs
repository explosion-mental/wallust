use std::{
    path::PathBuf,
    io::Write,
};

use wallust::{
    args::Cli,
    config::Config,
};

/// clap assertions
#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

/// setting contents to a const allows to use `{` and other formatting special chars in the file.
#[allow(non_upper_case_globals)]
const content: &str = include_str!("../wallust.toml");

/// test for a valid `--config_file` and for the provided file to be the new location
#[test]
fn config_file() {
    let mut tmp = tempfile::NamedTempFile::new().expect("init new temporal named pipe");

    write!(tmp, "{content}").expect("should write to tmp correctly");

    let config_file = Some(tmp.path().to_path_buf());

    let conf_dir = "~/.config";

    // serde + logic to find out the new config
    let c = Config::new(&PathBuf::from(conf_dir), config_file.as_deref(), None, false).expect("should deserialize wallust.toml");

    // config path directory should remain the same + an added `wallust/`
    assert_eq!(c.dir, PathBuf::from(conf_dir).join("wallust"));

    // c.file should be the new one provided
    assert_eq!(c.file, tmp.path().to_path_buf());

    tmp.close().expect("temporal named pipe should close successfully");
}

/// Test for `--config-dir` provided directory is used and should check `wallust.toml` inside it
#[test]
fn config_dir() {
    use std::fs::File;

    let tmp = tempfile::tempdir().expect("init new temporal named pipe");

    let joined = tmp.path().join("wallust.toml");
    let mut conf_tmp = File::create(joined).expect("should created a tmp file");
    write!(conf_tmp, "{content}").expect("should write to tmp correctly");

    let config_dir = Some(tmp.path().to_path_buf());

    let original_conf = "~/.config"; //pseudo "original" config
    let c = Config::new(&PathBuf::from(original_conf), None, config_dir.as_deref(), false).expect("should deserialize wallust.toml");

    // config path directory should NOT remain the "original_conf", but changed to the one provided by the cli (args.config_dir)
    assert_eq!(c.dir, tmp.path().to_path_buf());

    // config file should be inside the new provided dir
    assert_eq!(c.file, tmp.path().join("wallust.toml").to_path_buf());

    tmp.close().expect("temporal directory should close successfully");
}
