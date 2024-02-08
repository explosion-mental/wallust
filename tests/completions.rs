

/// completion dir
const COMPLETION_DIR: &str = "./completions";

#[test]
fn mk_completion() {
    use clap_complete::{generate_to, Shell};
    use clap::{ValueEnum, CommandFactory};

    // <https://docs.rs/clap/latest/clap/struct.Command.html>
    let mut cmd = wallust::args::Subcmds::command();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "wallust", COMPLETION_DIR).expect("generate_to FAILED");
    }

    // cmd.build();
    // let man = clap_mangen::Man::new(cmd);
    // let mut buffer: Vec<u8> = Default::default();
    // man.render(&mut buffer).unwrap();
    //
    // std::fs::write(std::path::Path::new("./").join("mybin.1"), buffer).unwrap();

    // man page generation
    // TODO I think I'm gonna have the man page written down in markdown..
    //
    // use std::path::Path;
    // use wallust::args::Subcmds;
    // use std::fs::File;
    // use std::io::Write;
    //
    // fn print(dir: &Path, app: &clap::Command) -> anyhow::Result<()> {
    //     // `get_display_name()` is `Some` for all instances, except the root.
    //     let name = app.get_display_name().unwrap_or_else(|| app.get_name());
    //     let mut out = File::create(dir.join(format!("{name}.1")))?;
    //
    //     clap_mangen::Man::new(app.clone()).render(&mut out)?;
    //     out.flush()?;
    //
    //     for sub in app.get_subcommands() {
    //         print(dir, sub)?;
    //     }
    //
    //     Ok(())
    // }
    //
    // cmd.build();
    //
    // print(Path::new("./"), &cmd).unwrap()
}
