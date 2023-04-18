# wallust - Generate colors from an image
[![crates io](https://img.shields.io/crates/v/wallust?style=flat-square&color=red)](https://crates.io/crates/wallust)
[![downloads](https://img.shields.io/crates/d/wallust?style=flat-square&color=yellow)](https://crates.io/crates/wallust)
[![license](https://img.shields.io/crates/l/wallust?style=flat-square)](https://codeberg.org/explosion-mental/wallust/src/branch/main/LICENSE)
[![dependency status](https://deps.rs/repo/codeberg/explosion-mental/wallust/status.svg?style=flat-square)](https://deps.rs/repo/codeberg/explosion-mental/wallust)
[![CodeBerg](https://img.shields.io/badge/Hosted_at-Codeberg-%232185D0?style=flat-square&logo=CodeBerg)](https://codeberg.org/explosion-mental/wallust)
<br>

![gif](https://explosion-mental.codeberg.page/img/other/wallust-v2.gif "wallust gif")
> sources: [rms by marco novo](https://stallman.org/photos/rms/pages/2.html) - [linus talking](https://en.wikipedia.org/wiki/File:Linus_Torvalds_talking.jpeg) - [pixels](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/pixels-l.webp) - [adwaita](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/adwaita-d.jpg)

**It is reccomended to clean the cache in a new major and minor release**

## Features
- Sets terminal colors sequences on all active terminals
- Config file at `~/.config/wallust/wallust.toml`:
	* templating integrated in a config file
	* backends, colorspaces and filters
	* configurable threshold
- Cache scheme palette at `~/.cache/wallust`

### Terminal color sequences
By default, `wallust` will send these sequences to all open terminals
(/dev/pts/). You can skip this with the `-s` or `--skip-sequences` flag.

When opening new terminals you will notice that the color sequences are not
applied. To solve this you can send the sequences yourself when your shell
opens. `wallust` will store the sequences in `~/.cache/wallust/sequences`, so
the usual way is to `cat ~/.cache/wallust/sequences` in your `.zshrc`,
`.bashrc`, etc.

### Templating & Config File
You can use `wallust` generated colors in your program by __template__ing them
in it's config file. Below an example:
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
Then add this file to `~/.config/wallust/` (e.g.
__~/.config/wallust/zathurarc__) and add a new entry to `wallust.toml`
```toml
[[entry]]
template = "zathurarc"
target = "~/.config/zathura/zathurarc"
```

**NOTE:** The template name doesn't have to match the target name, e.g. the
file could be named `sample.conf`, and thus the entry would have `template =
"sample.conf"`, but the target can remain the same.

## Usage
```
wallust my_wallpaper.png
```

## Installation
```
cargo install wallust
```

or, if you cloned the repo
```
cargo install --path .
```
or just build it (`cargo build --release`) and copy the binary to one folder
present in your `$PATH` like `/usr/local/bin`
```
cargo build --release
cp -f ./target/release/wallust /usr/local/bin
```

### NetBSD
If you are using NetBSD, a native package is available from the official repositories. To install it, simply run:
```
pkgin install wallust
```

## TODO
- **release binaries** - figure out woodkeeper codeberg CI

