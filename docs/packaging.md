# Packaging

Binary-based distros can grab the latest pre-compiled binary from the
[releases page](https://codeberg.org/explosion-mental/wallust/releases).

Source-based distros, if they wish to build `wallust` from source, must ensure
that the following dependencies are available:

- Build Dependencies:
	1. Rust (`cargo`, `rustc`)
    2. make (or install man pages and completions manually)
- Runtime Dependencies
    1. [`imagemagick`](https://imagemagick.org) is required **only** for the `wal`
       backend, such limiting should be mentined and considered an **optional**
       dependency, since all other backends work without it.
