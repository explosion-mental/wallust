# wallust - Generate colors from an image

![gif](https://explosion-mental.codeberg.page/img/screensus/wallust.gif "wallust gif")
> [^sources]

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


## TODO
- config/cli flag that allows to set the colors as variables in shell, like pywal `$color1` ...
- config/cli flag that allows to set the colors as Xresources values
- make `Colors` type not rely on lab color space, so the backends can include
  other color spaces like hsl/hsv or Oklab
- think about releasing binaries with CI

[^sources]:
1: <https://stallman.org/photos/rms/pages/2.html>
2: <https://en.wikipedia.org/wiki/File:Linus_Torvalds_talking.jpeg>
3: <>
