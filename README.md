# wallust - Generate colors from an image
[![crates io](https://img.shields.io/crates/v/wallust?style=flat-square&color=red)](https://crates.io/crates/wallust)
[![downloads](https://img.shields.io/crates/d/wallust?style=flat-square&color=yellow)](https://crates.io/crates/wallust)
[![license](https://img.shields.io/crates/l/wallust?style=flat-square)](https://codeberg.org/explosion-mental/wallust/src/branch/main/LICENSE)
[![dependency status](https://deps.rs/repo/codeberg/explosion-mental/wallust/status.svg?style=flat-square)](https://deps.rs/repo/codeberg/explosion-mental/wallust)
[![CodeBerg](https://img.shields.io/badge/Hosted_at-Codeberg-%232185D0?style=flat-square&logo=CodeBerg)](https://codeberg.org/explosion-mental/wallust)
<br>

![gif](https://explosion-mental.codeberg.page/img/other/wallust.gif "wallust gif")
> sources[^sources]

## Features
- use of templates and writing these out defined in the config file.
- different backends (currently only 2)
- cache values


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
- config/cli flag that allows to set the colors as variables in shell, like pywal `$color1` ...
- config/cli flag that allows to set the colors as Xresources values
- make `Colors` type not rely on lab color space, so the backends can include
  other color spaces like hsl/hsv or Oklab
- think about releasing binaries with CI

[^sources]: <https://stallman.org/photos/rms/pages/2.html> - <https://en.wikipedia.org/wiki/File:Linus_Torvalds_talking.jpeg> - <https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/pixels-l.webp> - <https://gitlab.gnome.org/GNOME/gnome-backgrounds/-/blob/main/backgrounds/adwaita-d.jpg>
