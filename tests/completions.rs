#![cfg(feature = "buildgen")]
use clap::{ValueEnum, CommandFactory};

// TODO maybe in the future, when I get my head along with workspaces just split up library and
// binary, move this `geneartion` like tests into `src/bin`

/// completion dir
const COMPLETION_DIR: &str = "./completions";

#[test]
fn mk_completion() {
    use clap_complete::{generate_to, Shell};

    // <https://docs.rs/clap/latest/clap/struct.Command.html>
    let mut cmd = wallust::args::Cli::command();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "wallust", COMPLETION_DIR).expect("generate_to FAILED");
    }
}
