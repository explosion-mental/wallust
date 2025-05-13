# wallust - Generate colors from an image
[//]: # (ANCHOR: badges)

[![crates-io](https://img.shields.io/crates/v/wallust?style=flat-square&color=red)](https://crates.io/crates/wallust)
[![downloads](https://img.shields.io/crates/d/wallust?style=flat-square&color=yellow)](https://crates.io/crates/wallust)
[![license](https://img.shields.io/crates/l/wallust?style=flat-square)](https://codeberg.org/explosion-mental/wallust/src/branch/main/LICENSE)
[![dependency-status](https://deps.rs/repo/codeberg/explosion-mental/wallust/status.svg?style=flat-square)](https://deps.rs/repo/codeberg/explosion-mental/wallust)
[![status-badge](https://ci.codeberg.org/api/badges/13317/status.svg)](https://ci.codeberg.org/repos/13317)
[![CodeBerg](https://img.shields.io/badge/Hosted_at-Codeberg-%232185D0?style=flat-square&logo=CodeBerg)](https://codeberg.org/explosion-mental/wallust)
[![MatrixChat](https://matrix.to/img/matrix-badge.svg)](https://matrix.to/#/#wal-lust:matrix.org)

[//]: # (ANCHOR_END: badges)
<br>

![gif](https://explosion-mental.codeberg.page/img/other/wallust-2.6.gif "wallust gif")
> sources: [adwaita](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/adwaita-d.jpg) - [scenic view of mountains](https://www.pexels.com/photo/scenic-view-of-mountains-during-dawn-1261728) - [rms by marco novo](https://stallman.org/photos/rms/pages/2.html) - [pixels](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/pixels-l.webp) - [linus talking](https://en.wikipedia.org/wiki/File:Linus_Torvalds_talking.jpeg)

**If coming from v2, please check [v3 breaking changes](./docs/v3.md).**

## Usage
```
wallust run my_wallpaper.png
```
_use `wallust -h` for an overview and `wallust --help` for a more detailed explanation_

## Docs

For ease of use you can check **detailed** docs with man pages (rather than `cmd -h`ing everytime):
- `man wallust`, information about terminal colors and **template syntax**;
- `man wallust.5`, config docs;
- `man wallust-subcommand`, displays a man page for _subcommand_.

There is also a [**web page**](https://explosion-mental.codeberg.page/wallust) for documentation! It's based on plain markdown, so you could also read it locally at `docs/` directory.

[//]: # (ANCHOR: feats)
## Features
- Includes [man pages](man/) and [completions](completions/)!
- Feature rich templating:
    * You can use a [subset of _Jinja2_](https://github.com/mitsuhiko/minijinja/blob/main/COMPATIBILITY.md) (default).
    * Alternatively, simply use the [_pywal_](https://github.com/dylanaraps/pywal/wiki/User-Template-Files#available-variables-and-syntax) syntax (requires selecting it on the config file).
- Sets terminal colors on all (or the current, `-u`) active terminals:
    * Windows: Adds a
      [color scheme for the windows terminal](https://learn.microsoft.com/en-us/windows/terminal/customize-settings/color-schemes#creating-your-own-color-scheme)
      by updating `settings.json` on Windows Terminal, to enable this scheme for the first time you will have to selected it manually
    * *NIX: ASCII escape sequences:
        - `/dev/pts/` on Linux
        - [`ps` to search active terminals](https://github.com/dylanaraps/pywal/pull/510) on OpenBSD
    * MacOS: iTerm2 sequences, `/dev/ttys00` on MacOS
- Cache scheme palettes, overwritten by `-w`:
    * Linux: `$XDG_CACHE_HOME` or `$HOME/.cache`
    * MacOs: `$HOME/Library/Caches`
    * Windows: `{FOLDERID_LocalAppData}`
- Read pywal/terminal-sexy colorschemes with `wallust cs`.
- Built-in [themes](https://codeberg.org/explosion-mental/wallust-themes) with ` wallust theme` (compile time feature).
- Configuration file at [`wallust.toml`](./wallust.toml) (but wallust can work without one!):
    * wallust checks for `~/.config/wallust/wallust.toml` for the config file, if not found it will use default implementations.
    * Configuration variables are avaliable as cli flags.
	* Configurable methods for backends, colorspaces, palettes and threshold.
    * OS dependant path:
        - Linux: `$XDG_CONFIG_HOME` or `$HOME/.config`
        - MacOs: `$HOME/Library/Application Support`
        - Windows: `{FOLDERID_RoamingAppData}`

| Methods    | Description |
|------------|-------------|
| Backends   | How to extract the colors from the image. (e.g [pywal uses convert](https://github.com/dylanaraps/pywal/blob/236aa48e741ff8d65c4c3826db2813bf2ee6f352/pywal/backends/wal.py#L14)) |
| ColorSpace | Get the most prominent color, and sort them according to the `Palette`, configurable by a _threshold_ |
| Palette    | Makes a scheme palette with the gathered colors, (e.g. sets light background) |

[//]: # (ANCHOR_END: feats)

# Installation

You can see if your distro has **wallust** in their repos by the following
chart. For detail information you can check some [distro installation instruction](./docs/installation/distro.md)
that the maintainers have left.

[//]: # (ANCHOR: repology)

<a href="https://repology.org/project/wallust/versions">
  <img align="right" width="192" src="https://repology.org/badge/vertical-allrepos/wallust.svg">
</a>

[//]: # (ANCHOR_END: repology)

[//]: # (ANCHOR: installation-src)

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

[//]: # (ANCHOR_END: installation-src)

# Background
I've started this tool mainly for _speed_ reasons given that I use a
[keybinding](https://codeberg.org/explosion-mental/demwm/src/commit/cd43bd6c16a90e32dc1c22ec499d9aaff497f04a/config.h#L346)
that [runs pywal](https://codeberg.org/explosion-mental/demwm/src/commit/cd43bd6c16a90e32dc1c22ec499d9aaff497f04a/demwm_random_wall#L19)
with a random wallpaper image, this resulted in a noticeable delay in between,
which was really annoying as I changed my wallpaper.

Now I know that pywal uses image magick `convert` to gather the colors, which
caused the _slowness_ in pywal, since `convert` takes some time by itself to
gather the colors.

Wallust can also use `convert` with the `wal` backend, but it's even more
powerful since it brings up integrated native methods (no need for external
commands, like `convert`, but are nice if you have it), and even much more
sofisticated algorithms like `kmeans` or the SIMD backend `fast_resize`. This
made the need to let the user decide what fit best, and not hardcode one way of
reading the image.

While the goal was focused on speed, the use case move on to upgrade
functionality that both wallust and the _archived_ python tool shared.

I use rust given the great wide library (crates) that it offered and it's
native capabilities. I also tried [rewriting pywal in C](https://github.com/explosion-mental/wast)
after watching a tsoding video where he implmements a histogram in C for
manipulating an image. That was the push I needed to start this journey on this
rusty color theoryish codebase.


# Related
- [wallust-themes - built in wallust colorschemes](https://codeberg.org/explosion-mental/wallust-themes)
- [wallust-templates - some templates for known programs](https://codeberg.org/explosion-mental/wallust-templates)
- [pywal - 🎨Generate and change color-schemes on the fly](https://github.com/dylanaraps/pywal)
- [pywal16 - 16 colors fork of pywal](https://github.com/eylles/pywal16)
- [wpgtk - 🎴a colorscheme, wallpaper and template manager for *nix](https://github.com/deviantfero/wpgtk)
- [wal-theme-picker - pick the best theme for the image (_rather than generating one_)](https://github.com/drybalka/wal-theme-picker)
- [pigmnts - 🎨Color palette generator from an image using WebAssesmbly and Rust](https://github.com/blenderskool/pigmnts)
- [Chameleon - 🦎Theme your linux system to match any image](https://github.com/GideonWolfe/Chameleon)
- [lule_bash - Genretare all 255 colors from wallpapers](https://github.com/warpwm/lule_bash)
- [lule - `lule_bash` rewriten for efficiency](https://github.com/warpwm/lule)
- [using vscode-wal-theme with `wallust`](https://github.com/dlasagno/vscode-wal-theme/issues/23)
- [base16 - Framework for Tomorrow styled themes](https://github.com/chriskempson/base16)
- [flavours -  🎨💧An easy to use base16 scheme manager that integrates with any workflow](https://github.com/Misterio77/flavours)
- [oxidec - Eye-candy manager written in Rust](https://github.com/mrtnvgr/oxidec)
- [raventhemer - A theme manager and switcher for desktop linux](https://git.sr.ht/~nicohman/raven)
- [rose-pine _Issue #2_ - Ideas with using the whole 15 color palette](https://github.com/rose-pine/xresources/issues/2)
