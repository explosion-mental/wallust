# Template Filters

The Jinja2 format calls them 'filters', making a distincion from 'functions'.

Currently I haven't implemented any **function** because I haven't found a usecase (yet?).

## Filters that take an _UNSIGNED INT_

### alpha_hexa
Displays alpha value as
[**hexadecimal color code**](https://gist.github.com/lopspower/03fb1cc0ac9f32ef38f4)
(e.g "`{{ 100 | alpha_hexa }}`" outputs '`FF`').
This can only be used with numbers from 0 to 100, so you are free to use the
variable _alpha_ with this filter.

## Filters that take the _color_ type
Functions that only work with colors. These can be applied to a _color_, which
can be the COLOR variables listed in "Variables" section, or a literal color
like "`#0A0B0C`". These functions return a color in the mentioned format (hex
rgb, like "`#000000`"), unless written otherwise (like rgb, rgba, the other
filters that explicitly say it's output format). This allows to apply multiple
filters at a time.

**_Note:_**
If an 'alpha' value is mentioned, it's defined in the config file, as a cli flag and by default it's value is '100'.


### hexa
Outputs the color in `hexa` format: e.g "`#0A0B0CFF`", where '`FF`' is the alpha value. .

**_Note:_** This, internally uses `alpha_hexa` filter from above.

Example:
```
{{ color5 | hexa }}
```


### xrgb
Output the color in `xrgb`, separated by slashes. (e.g "0A/0B/0C")

### strip
Output the color in `hex`, just like by default, but removes the leading `#`. (e.g. "0A0B0C")

### rgb
Output the color in `rgb`, separated by comas. (e.g. "10,11,12")

### red
Outputs only the red value. (e.g. "10")

### green
Outputs only the green value. (e.g. "11")

### blue
Outputs only the blue value. (e.g. "12")

### rgbf
Output the color in `rgb` **floating point** from 0.0 to 1.0 with 4 decimals at max.
The values are separated by comas.

### redf
Outputs only the red value as a **float**.

### greenf
Outputs only the green value as a **float**.

### bluef
Outputs only the blue value as a **float**.

### complementary
Returns the respective complementary color.

### blend
Takes another **color** as input, to blend it for the filtered color.

Example:
```
{{ color2 | blend(color0) | blend("#EEDDFF") }}
```

### lighten
Takes a **float** (decimal value) as input, from `0.1` to `1.0`, that corresponds
to the amount to lighten the color by.

Example:
```
{{ color0 | lighten(0.2) }}
```

### darken
Takes a **float** (decimal value) as input, from `0.1` to `1.0`, that corresponds to the amount to darken the color by.

### saturate
Takes a **float** (decimal value) as input, from `0.1` to `1.0`, that corresponds to the amount to saturate the color by.
