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

## Usage
```
wallust my_wallpaper.png
```
_use `wallust -h` for an overview and `wallust --help` for a more detailed explanation_

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
- Can read pywal colorschemes with `cs` subcommand
- Built-in [pywal themes](https://github.com/dylanaraps/pywal/tree/master/pywal/colorschemes) with the `theme` subcommand (can be disabled with compile-time features)

### Backends
This let's you choose a way to read the image, as in read a file and return
it's rgb8 bytes. This can be done the _usual_ way using imagemagick
[convert](https://imagemagick.org/script/command-line-processing.php) tool,
just like how
[`pywal` does it](https://github.com/dylanaraps/pywal/blob/236aa48e741ff8d65c4c3826db2813bf2ee6f352/pywal/backends/wal.py#L14),
which `wallust` can also do (this requires the actual CLI program `convert`
installed), or other methods **dependency free**.


### ColorSpace
This takes the bytes read from the backend and returns the most prominent one
and sorts them acording to the filter.


This is a picky configurable section, since there isn't much difference in
between the generated palettes with diverging colorspaces. However I think it's
interesting to use other color spaces like OkLab (a more precise hardcoded
algo) or HSL (which pywal originally uses).

#### Threshold
This is used inside the colorspace itself, the usual **good** number is 11.

 Number  | Description
---------|------------
 <= 1    | Not perceptible by human eyes.
 1 - 2   | Perceptible through close observation.
 2 - 10  | Perceptible at a glance.
 11 - 49 | Colors are more similar than opposite
 100     | Colors are exact opposite

### Filter
This uses the colors returned by the colorspace and orders them in a way that
makes sense, as in making sure that the contrast matches or the background is a
certain type. All of these depend on the filter being used, each of them are
described in the default config.

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
You can find examples at
[pywal templates](https://github.com/dylanaraps/pywal/tree/master/pywal/templates)
or
[wpgtk templates](https://github.com/deviantfero/wpgtk-templates)

**NOTE:** The template name doesn't have to match the target name: e.g. the
file could be named `sample.conf`, and thus the entry would have `template =
"sample.conf"`, but the target can remain the same, e.g. `target = "~/.config/zathurarc"`.

#### Variables and Methods
- `{wallpaper}`: The full path to the current wallpaper.
- `{alpha}`: displays 100, this is here to be compatible with pywal templates.
- `{var}`: Output the color in `hex`.
- `{var.rgb}`: Output the color in `rgb`.
- `{var.rgba}`: Output the color in `rgba`.
- `{var.xrgba}`: Output the color in `xrgb`.
- `{var.strip}`: Output the color in `hex` (without a `#`).
- `{var.red}`: Output the red value.
- `{var.green}`: Output the green value.
- `{var.blue}`: Output the blue value.

Where `var` can be `color0` - `color15`, `background`, `foreground` and `cursor`.


## Installation

<a href="https://repology.org/project/wallust/versions">
  <img align="right" width="192" src="https://repology.org/badge/vertical-allrepos/wallust.svg">
</a>

### Distros Packages
#### NetBSD
If you are using NetBSD, a native package is available from the official repositories. To install it, simply run:
```
pkgin install wallust
```

#### Nix
If you are using Nix, a native package is available for the [unstable channel][nix-search].

Install it for your profile:
```
nix-env -iA nixos.wallust # change `nixos` for `nixpkgs`, if on a non-NixOS system
```

Try it with `nix-shell`
```
nix-shell -p wallust
```

[nix-search]: <https://search.nixos.org/packages?channel=unstable&from=0&size=1&sort=relevance&type=packages&query=wallust>


### Binary
Go to the [releases](https://codeberg.org/explosion-mental/wallust/releases)
and download the `tar.gz` file, which contains a binary for musl, so it should
work for most *nix platforms.

```
tar -xf wallust-TARGET.tar.gz
```

### Build from source
_The master branch **is** stable_

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

## Contribute!
**Use the [dev](https://codeberg.org/explosion-mental/wallust/src/branch/dev) branch**


Show some of your taste by adding a
[backends](https://codeberg.org/explosion-mental/wallust/src/branch/master/src/backends),
[colorspaces](https://codeberg.org/explosion-mental/wallust/src/branch/master/src/colorspaces)
and/or
[filters](https://codeberg.org/explosion-mental/wallust/src/branch/master/src/filters).


Having thoughts or suggestios is also very welcome.

## TODOs
for more, grep the src for TODO `rg TODO`
- release binaries with a CI, figure out woodkeeper codeberg CI
- Think about using [k means algo](https://en.wikipedia.org/wiki/K-means_clustering)
  similar to [pigmnts](https://github.com/blenderskool/pigmnts) (just without seg faulting :p)
- use `thiserror` for errors in the modules (there aren't that many)

## Related
- [pywal - 🎨Generate and change color-schemes on the fly](https://github.com/dylanaraps/pywal)
- [pywal16 - 16 colors fork of pywal](https://github.com/eylles/pywal16)
- [wpgtk - 🎴a colorscheme, wallpaper and template manager for *nix](https://github.com/deviantfero/wpgtk)
- [wal-theme-picker - pick the best theme for the image (_rather than generating one_)](https://github.com/drybalka/wal-theme-picker)
- [pigmnts - 🎨Color palette generator from an image using WebAssesmbly and Rust](https://github.com/blenderskool/pigmnts)
- [Chameleon - 🦎Theme your linux system to match any image](https://github.com/GideonWolfe/Chameleon)
- using [vscode-wal-theme with `wallust`](https://github.com/dlasagno/vscode-wal-theme/issues/23)
