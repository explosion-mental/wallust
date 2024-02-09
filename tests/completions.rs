use clap::{ValueEnum, CommandFactory};

/// completion dir
const COMPLETION_DIR: &str = "./completions";

#[test]
fn mk_completion() {
    use clap_complete::{generate_to, Shell};

    // <https://docs.rs/clap/latest/clap/struct.Command.html>
    let mut cmd = wallust::args::Subcmds::command();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "wallust", COMPLETION_DIR).expect("generate_to FAILED");
    }
}
