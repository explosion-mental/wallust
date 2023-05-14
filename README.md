# wallust - Generate colors from an image
[![crates io](https://img.shields.io/crates/v/wallust?style=flat-square&color=red)](https://crates.io/crates/wallust)
[![downloads](https://img.shields.io/crates/d/wallust?style=flat-square&color=yellow)](https://crates.io/crates/wallust)
[![license](https://img.shields.io/crates/l/wallust?style=flat-square)](https://codeberg.org/explosion-mental/wallust/src/branch/main/LICENSE)
[![dependency status](https://deps.rs/repo/codeberg/explosion-mental/wallust/status.svg?style=flat-square)](https://deps.rs/repo/codeberg/explosion-mental/wallust)
[![CodeBerg](https://img.shields.io/badge/Hosted_at-Codeberg-%232185D0?style=flat-square&logo=CodeBerg)](https://codeberg.org/explosion-mental/wallust)
[![MatrixChat](https://matrix.to/img/matrix-badge.svg)](https://matrix.to/#/#wal-lust:matrix.org)
<br>

![gif](https://explosion-mental.codeberg.page/img/other/wallust-v2.3.gif "wallust gif")
> sources: [rms by marco novo](https://stallman.org/photos/rms/pages/2.html) - [linus talking](https://en.wikipedia.org/wiki/File:Linus_Torvalds_talking.jpeg) - [pixels](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/pixels-l.webp) - [adwaita](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/adwaita-d.jpg)

**It is recommended to clean the cache in a new major and minor release** _but is not required_


If you don't have a config file, `wallust` will generate the
[default config file](https://codeberg.org/explosion-mental/wallust/src/branch/master/wallust.toml)
for you.

## Features
- Sets terminal colors sequences on all active terminals
- Respects directory structure by platform:
    * Cache:
        - Linux: `$XDG_CACHE_HOME` or `$HOME/.cache`
        - MacOs: `$HOME/Library/Caches`
        - Windows: `{FOLDERID_LocalAppData}`
    * Config:
        - Linux: `$XDG_CONFIG_HOME` or `$HOME/.config`
        - MacOs: `$HOME/Library/Application Support`
        - Windows: `{FOLDERID_RoamingAppData}`
- Configuration file, documented at `wallust.toml` of this repo:
	* optional templating integrated in a config file
	* backends, colorspaces and filters
	* configurable threshold
- Cache scheme palettes

### Terminal color sequences
By default, `wallust` will send these sequences to all open terminals
(/dev/pts/). You can skip this with the `-s` or `--skip-sequences` flag.

When opening new terminals you will notice that the color sequences are not
applied. To solve this you can send the sequences yourself when your shell
opens. `wallust` will store the sequences in the cache directory as a file
called `sequences`, the usual way is to `cat ~/.cache/wallust/sequences` in
your `.zshrc`, `.bashrc`, etc.

### Templating [OPTIONAL]
You can use `wallust` generated colors in a program by _templating_ the colors
in it's config file, like the following example:
```
# zathurarc config file

#keybindings
...

# colors
set default-bg     "{color2}"
set default-fg     "{foreground}"
set statusbar-bg   "{color4}"
set statusbar-fg   "{color6}"
set inputbar-bg    "{color1}"
```
Then add this file to `~/.config/wallust/` e.g. _~/.config/wallust/zathurarc_
(config directory defined by the platform) and add a new entry to `wallust.toml`
```toml
[[entry]]
template = "zathurarc"
target = "~/.config/zathura/zathurarc"
```

**NOTE:** The template name doesn't have to match the target name: e.g. the
file could be named `sample.conf`, and thus the entry would have `template =
"sample.conf"`, but the target can remain the same, e.g. `target = "~/.config/zathurarc"`.

## Usage
```
wallust my_wallpaper.png
```

## Installation
Keep in mind that the git repo is constantly updated, if you wanna use git,
`checkout` to a stable version.

### Build from source
#### From this repo
Go to the [releases](https://codeberg.org/explosion-mental/wallust/releases)
page and download the `.zip` or `.tar.gz` repository. After extracting the contents,
go to the directory (`cd MAYOR.MINOR.PATCH`).

Then you can do the following, which moves the binary into your `$CARGO_HOME/bin`
```
cargo install --path .
```

or build it and copy the binary to one folder present in your `$PATH` like
`/usr/local/bin`
```
cargo build --release
cp -f ./target/release/wallust /usr/local/bin
```

#### From crates.io
```
cargo install wallust
```
This will use the lastest version

### NetBSD
If you are using NetBSD, a native package is available from the official repositories. To install it, simply run:
```
pkgin install wallust
```

## TODO
for more, grep the src for TODO `rg TODO`
- **release binaries** - figure out woodkeeper codeberg CI
- Think about using [k means algo](https://en.wikipedia.org/wiki/K-means_clustering)
  similar to [pigmnts](https://github.com/blenderskool/pigmnts) (just without seg faulting :p)
- use `thiserror` for errors in the modules (there aren't that many)

## Related
- [pywal - 🎨Generate and change color-schemes on the fly](https://github.com/dylanaraps/pywal)
- [pywal16 - 16 colors fork of pywal](https://github.com/eylles/pywal16)
- [wpgtk - 🎴a colorscheme, wallpaper and template manager for *nix](https://github.com/deviantfero/wpgtk)
- [wal-theme-picker - pick the best theme for the image (_rather than generating one_)](https://github.com/drybalka/wal-theme-picker)
- [pigmnts - 🎨Color palette generator from an image using WebAssesmbly and Rust](https://github.com/blenderskool/pigmnts)
