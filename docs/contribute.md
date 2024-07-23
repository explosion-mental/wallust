# Contribute!
**Use the [dev](https://codeberg.org/explosion-mental/wallust/src/branch/dev) branch**

Show some of your taste by adding a [backend](./src/backends/mod.rs),
[colorspace](./src/colorspaces/mod.rs), [scheme palettes](./src/palettes/mod.rs),
and/or a [custom theme](https://codeberg.org/explosion-mental/wallust-themes).

Having design ideas or suggestios is also very welcome.


## TODOs
for more, grep the src for TODO `rg TODO`

- `wallust init`, which will scan your system for common aplication in which
 there is an avaliable theme template at
 [wallust-templates](https://codeberg.org/explosion-mental/wallust-templates).
 If so, it will automatically fetch those templates, add it in your toml
 config and configure accorndingly.

- Investigate more about woodpecker stuff.

- Allow to have more than 6 (MIN_COLS) passed to `palettes`. This requires some
 comunication to be exchanged between `ColorSpace <-> Palette` modules.

- Learn more from "Material You" implementations, what I've seen is that it
  looses some colors and hues to mantain contrast.
