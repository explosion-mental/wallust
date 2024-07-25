# color_space
What colorspace to use to gather the most prominent colors.

| Name | Description |
|------|-------------|
**lab** | Uses Cie L a b color space. *(mixed)*
**lch** | CIE Lch, you can understand this color space like LAB but with chrome and hue added, which Could help when sorting. *(mixed)*

There is one variant: **mixed**, which mixes colors when collecting them into a histogram.

<hr>

To edit this value:
- **Config file**: `color_space = "lchmixed"`
- **Cli**: `wallust run image.png --colorspace lchmixed`
