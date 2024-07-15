# Configuration File

Some general variables that can be configured in the `wallust.toml` file.

**All variables are optional**, as wallust can work without a config
file and choose default implementations.

Keep in mind that backend, palette and colorspace share possible values
with the cli.

## backend
Allows you to choose which method to use in order to parse the image.

> **full**
>
>    Read and return the whole image pixels more precision slower.
>
> **resized**
>
>    Resizes the image before parsing mantaining it s aspect ratio.
>
> **wal**
>
>    Uses image magick convert to generate the colors like pywal.
>
> **thumb**
>
>    Faster algo hardcoded to x no ratio respected.
>
> **fastresize**
>
>    A much faster resize algo that uses SIMD. For some reason it fails
>     on some images where resized doesn t for this reason it doesn t
>     replace but rather it s a new option.
>
> **kmeans**
>
>    Kmeans is an algo that divides and picks pixels all around the
>     image. Requires more tweaking and more in depth testing but for
>     the most part it just werks.

## color_space

:   What colorspace to use to gather the most prominent colors.

> **lab**
>
> :   Uses Cie L a b color space. *(mixed)*
>
> **lch**
>
> :   CIE Lch, you can understand this color space like LAB but with
>     chrome and hue added, which Could help when sorting. *(mixed)*

> There is one variant: **mixed**, which mixes colors when collecting
> them into a histogram.

## threshold

Wallust automatically uses the **best threshold** (by my testings) if this variable isn't defined.

>    If you really want to define this variable, keep in mind the
>    following. Thershold is the **difference between similar colors** ,
>    used inside the colorspace. While each colorspace may have different
>    results with different thresholds, meaning **you should try which
>    one works for you best** , an "overall" and general table looks
>    like this:

| Number | Description |
|--------|-------------|
1 | Not Perceptible by human eyes.
1 \- 2|Perceptible through close observation.
2 \- 10|Perceptible at a glance.
11 \- 49|Colors are more similar than opposite.
100|Colors are exact opposite.

## palette
Uses the colors gathered from `color_space` in a way that makes sense,
resulting in a scheme palette.

> **dark**
>
>    Dark colors dark background and light contrast. *(16, comp,
>     comp16)*
>
> **harddark**
>
>    Same as dark with hard hue colors. *(16, comp, comp16)*
>
> **light**
>
>    Light bg dark fg. *(16, comp, comp16)*
>
> **softdark**
>
>    Variant of softlight uses the lightest colors and a dark
>     background could be interpreted as dark inversed. *(16, comp,
>     comp16)*
>
> **softlight**
>
>    Light with soft pastel colors counterpart of harddark. *(16, comp,
>     comp16)*


### palette variants
There are some variants to the principal palettes schemes which you can use by
appending the variant to the name e.g. '`dark16`', '`lightcomp`', '`harddarkcomp16`'
and so on, each palette indicates, in parenthesis, which variants are
avaliable. A description of the overall variants:

> **16**
>
>    Makes shades of colors, creating the ilusion of "16 different
>     colors".
>
> **comp**
>
>    Stands for Complementary and completly changes the palette to
>     it's complementary counterpart.
>
> **comp16**
>
>    Complementary palette with 16 shades.

## check_constrast
Ensures a "readable contrast".
Should only be enabled when you notice an unreadable contrast frequently
happening with your images. The reference color for the contrast is the
background color. (default: **disabled**)


## fallback_generator
This field chooses a method to use when the gathered colors aren't enough:

    **interpolation**

    :   (default) Tries to pick two colors and built gradients over them
        **complementary** Uses the complementary colors of two colors,
        or more (if needed), colors.

## saturation
Color saturation, usually something higher than 50 increases the saturation and
below decreases it (on a scheme with strong and vivid colors).

_Possible values:_ 1 - 100 (default: **disabled**)

## alpha
Alpha value for templating (default: **100**)

# Templates

Templates are optional and defined inside the `[templates]` header. Here it's
recommended to use single quotes (`'`) instead of double quotes (`"`) since the
first one, by the toml format, ignores backslashes (`\`) as escape codes,
allowing you to define Widows like paths (e.g. `'C:\Users\Desktop\'`)

## template
A relative path that points to a file where wallust.toml is located, usually at
`~/.config/wallust/`. This file can also be a directory, which will be
templated **non-recursively**.

## target
Absolute path in which to place a file with generated templated values. This
field CAN expand the \`\~\` as the \`\$HOME\` enviromental variable. If
*template* is a directory, this **must** correspond and be one.

## pywal _(optional)_
Indicates to treat **template** as a pywal template, using `{variable}` syntax. (default: **false**)

# Example
Below is a simple example exahusting all possible cases. All the format
is correct:

```toml
backend = "full"
color_space = "lab"
threshold = "20"
palette = "softdark"
check_contrast = true

[templates]
# dunst templates
dunst.template = "dunstrc.monitor"
dunst.target = "~/.config/dunst/dunstrc"

# one liner for zathura
zathura = { template = 'zath', target = '~/.config/zathura/zathurarc' }

# even a shorter way
glava = { src = 'glava.glsl', target = '~/.config/glava/rc.glsl' }

# or splited, like dunst
res.src = "xres"
res.dst = "~/.config/Xresources"

# old times, good times (here I put old pywal templates)
# Also, note that the name doesn't matters (as long as it isn't already defined)
# NOTE that both src and dst are directories
pywal = { src = "templates/", dst = '~/.cache/wal/', pywal = true }
```
