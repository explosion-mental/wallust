# Palette
Uses the colors gathered from `color_space` in a way that makes sense,
resulting in a scheme palette.

| Name | Description |
|------|-------------|
**ansidark**  | Dark ansi colors, works best with lchansi and orders it's colors to preserve a constant tty like order: `color0` -> black, `color1` -> redish, `color2` -> greenish, and so on.
**dark**      | Dark colors dark background and light contrast. *(16, comp, comp16)*
**harddark**  | Same as dark with hard hue colors. *(16, comp, comp16)*
**light**     | Light bg dark fg. *(16, comp, comp16)*
**softdark**  | Variant of softlight uses the lightest colors and a dark background could be interpreted as dark inversed. *(16, comp, comp16)*
**softlight** | Light with soft pastel colors counterpart of harddark. *(16, comp, comp16)*


## Palette Variations
There are some variants to the principal palettes schemes which you can use by
appending the variant to the name e.g. '`dark16`', '`lightcomp`', '`harddarkcomp16`'
and so on, each palette indicates, in parenthesis, which variants are
avaliable.

| Name | Description |
|------|-------------|
**16**     | Makes shades of colors, creating the ilusion of _16 different colors_.
**comp**   | Stands for **Comp**lementary and completly changes the palette to it's complementary counterpart.
**comp16** | Complementary palette with 16 shades, basically a combination of the above.

## Overview
Below, is a complete overview of all palette schemes:

{{#include ./palette-table.md}}

<hr>

To edit this value:
- **Config file**: `palette = darkcomp16`
- **Cli**: `wallust run image.png --palette darkcomp16`
