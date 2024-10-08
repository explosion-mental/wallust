# color_space
What colorspace to use to gather the most prominent colors.

| Name | Description |
|------|-------------|
**lab** | Uses Cie L a b color space. *(mixed and ansi)*
**lch** | CIE Lch, you can understand this color space like LAB but with chrome and hue added, which Could help when sorting. *(mixed)*

There are two variants:
- **mixed**, which mixes colors when collecting them into a histogram.
- **ansi**, Tries to get a full color pallete similar to the one of a tty, this works best with `ansidark` [palette](./palette.md).

## Overview
Below, is a complete overview of all colorspaces variations:

{{#include ./colorspace-table.md}}

<hr>

To edit this value:
- **Config file**: `color_space = "lchmixed"`
- **Cli**: `wallust run image.png --colorspace lchmixed`
