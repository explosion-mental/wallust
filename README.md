# wallust - Generate colors from an image
[![crates io](https://img.shields.io/crates/v/wallust?style=flat-square&color=red)](https://crates.io/crates/wallust)
[![downloads](https://img.shields.io/crates/d/wallust?style=flat-square&color=yellow)](https://crates.io/crates/wallust)
[![license](https://img.shields.io/crates/l/wallust?style=flat-square)](https://codeberg.org/explosion-mental/wallust/src/branch/main/LICENSE)
[![dependency status](https://deps.rs/repo/codeberg/explosion-mental/wallust/status.svg?style=flat-square)](https://deps.rs/repo/codeberg/explosion-mental/wallust)
[![CodeBerg](https://img.shields.io/badge/Hosted_at-Codeberg-%232185D0?style=flat-square&logo=CodeBerg)](https://codeberg.org/explosion-mental/wallust)
[![MatrixChat](https://matrix.to/img/matrix-badge.svg)](https://matrix.to/#/#wal-lust:matrix.org)
<br>

![gif](https://explosion-mental.codeberg.page/img/other/wallust-2.6.gif "wallust gif")
> sources: [adwaita](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/adwaita-d.jpg) - [scenic view of mountains](https://www.pexels.com/photo/scenic-view-of-mountains-during-dawn-1261728) - [rms by marco novo](https://stallman.org/photos/rms/pages/2.html) - [pixels](https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/pixels-l.webp) - [linus talking](https://en.wikipedia.org/wiki/File:Linus_Torvalds_talking.jpeg)

**If comming from v2, please check [v3 breaking changes](v3.md).**

## Usage
```
wallust run my_wallpaper.png
```
_use `wallust -h` for an overview and `wallust --help` for a more detailed explanation_

For ease of use you can check **detailed** docs with man pages (rather than `cmd -h`ing everytime):
- `man wallust`, information about terminal colors and **template syntax**;
- `man wallust.5`, config docs;
- `man wallust-subcommand`, displays a man page for _subcommand_.

Just in case there are also versions of some man pages as plain ascii at the
[wiki](https://codeberg.org/explosion-mental/wallust/wiki).

## Features
- Includes [man pages](man/) and [completions](completions/)!
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
- Optional Configuration file, [`wallust.toml`](./wallust.toml):
    * wallust checks for `~/.config/wallust/wallust.toml` for the config file, if not found it will use default implementations.
    * Configuration variables are avaliable as cli flags.
	* **Optional** templating with a [subset of _Jinja2_](https://github.com/mitsuhiko/minijinja/blob/main/COMPATIBILITY.md) or [_pywal_](https://github.com/dylanaraps/pywal/wiki/User-Template-Files#available-variables-and-syntax) syntax if selected.
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


## Installation
<a href="https://repology.org/project/wallust/versions">
  <img align="right" width="192" src="https://repology.org/badge/vertical-allrepos/wallust.svg">
</a>

`wallust` doesn't require third party packages, but has an **optional**
dependency: [`imagemagick`](https://imagemagick.org) to use the `wal` backend
(just like pywal). Other methods are built in.

### Distros Packages
#### NetBSD
If you are using NetBSD, a native package is available from the official repositories. To install it, simply run:
```
pkgin install wallust
```

#### Nix

If you are using Nix, a native package is [available][nix-search].

Install it for your profile:

```bash
nix-env -iA nixos.wallust # change `nixos` for `nixpkgs`, if on a non-NixOS system
```

Try it with `nix-shell`

```bash
nix-shell -p wallust
```

Add the following Nix code to your NixOS Configuration, usually located in `/etc/nixos/configuration.nix`

```nix
  environment.systemPackages = [
    pkgs.wallust
  ];
```

If you are using flakes, you can directly use this repo to get the latest release:

First add this to your `flake.nix`

```nix
  inputs.wallust.url = "git+https://codeberg.org/explosion-mental/wallust?ref=master";
```

Then in your `configuration.nix`

```nix
  environment.systemPackages = [
    inputs.wallust.packages.${pkgs.system}.wallust
  ];
```

You can change ref to `dev` branch to get the latest nightly.

[nix-search]: <https://search.nixos.org/packages?channel=unstable&from=0&size=1&sort=relevance&type=packages&query=wallust>

#### Arch User Repository (AUR)
Using an Arch based distro, you can use the [wallust](https://aur.archlinux.org/packages/wallust) or [wallust-git](https://aur.archlinux.org/packages/wallust-git) packages.

- `wallust` fetches the latest **stable version** from `static.crates.io`, which mirrors the `master` branch. **Prefer this package.**
- `wallust-git` fetches the latest **unstable version** from the `dev` branch.

Either can be installed on an Arch based distro with the following commands:
```bash
git clone https://aur.archlinux.org/wallust.git # Or wallust-git.git
cd wallust # or wallust-git
makepkg -si
```

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

#### From crates.io
```
cargo install wallust
```
This will use the lastest version

## Packaging

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

### Makefile
Using `make` is _optional_ if you know your way into cargo and **can** accept
the job to **manually install man pages, completions and the binary**.

I've only added a `Makefile` to simplify _installing_ these assets, as well as
the binary. By default  `make` uses native compilation, you can define your
wished target like this:

**Building**
```
$ TARGET=x86_64-pc-windows-gnu make install CARGOFLAGS="--release --target=$TARGET"
```

**Installing**
```
# TARGET=x86_64-pc-windows-gnu make CARGOFLAGS="--release --target=$TARGET" RELEASE="target/$TARGET/release"
```

Don't forget that `make` by itself runs `cargo` in order to built the binary.
It's common on projects that use make to split building in two steps, given
that `make install` requires permissions to write on `$DESTDIR$PREFIX`.

## Contribute!
**Use the [dev](https://codeberg.org/explosion-mental/wallust/src/branch/dev) branch**

Show some of your taste by adding a [backend](./src/backends/mod.rs),
[colorspace](./src/colorspaces/mod.rs), [scheme palettes](./src/palettes/mod.rs),
and/or a [custom theme](https://codeberg.org/explosion-mental/wallust-themes).

Having design ideas or suggestios is also very welcome.

## TODOs
for more, grep the src for TODO `rg TODO`
- think about creating a user friendly `mdbook` like
- Learn more from "Material You" implementations, what I've seen is that it
  looses some colors and hues to mantain contrast.

## Background
I've started this tool mainly for _speed_ reasons[0], since I have a
[keybinding](https://codeberg.org/explosion-mental/demwm/src/commit/cd43bd6c16a90e32dc1c22ec499d9aaff497f04a/config.h#L346)
that [runs pywal](https://codeberg.org/explosion-mental/demwm/src/commit/cd43bd6c16a90e32dc1c22ec499d9aaff497f04a/demwm_random_wall#L19)
to a random wallpaper with a noticeable delay. I tried ["rewriting" pywal in C](https://github.com/explosion-mental/wast)
after watching a tsoding video where he implmements a histogram in C for manipulating an image. That was the little push I needed
start this journey.

[0]: While that was the goal, big images are still gonna be big no matter the language you write your implementation, that's why there are different backends for different tastes :).

## Related
- [wallust-themes - built in wallust colorschemes](https://codeberg.org/explosion-mental/wallust-themes)
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
