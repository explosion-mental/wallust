# Building from source

## With cargo
```
cargo install wallust
```
This will use the lastest (non pre-release) version.

## With git
Simply `git clone https://codeberg.org/explosion-mental/wallust`.

Recommended way is to use the `Makefile`, since this will install man pages and completions.
1. Edit `Makefile` to meet your local setup (should be fine as is for most linux distros).
2. Build it with `make`
3. Install wallust (if necessary as root): `make install`

Optionally, installing **only** the binary can be done with the following,
which moves the binary into your `$CARGO_HOME/bin`:
```
cargo install --path .
```

or build it and copy the binary to one folder present in your `$PATH` like
`/usr/local/bin`
```
cargo build --release
cp -f ./target/release/wallust /usr/local/bin
```
