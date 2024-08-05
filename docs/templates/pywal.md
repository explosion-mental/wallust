# Pywal Template Compatibility
You can enable pywal like syntax in the config file with `pywal = true`.
 wallust (5)

The syntax is simple, but more variables are added given that it's engine and
spec doesn't support runtime evaluation functions.

While the implementation is simple enough to be added in wallust, it's use is discoraged.

## Syntax
The syntax logic is simply "Find and Replace" like:

```
somevariable = {color2}
anothervariable = {color8.rgb}
```

Don't forget to visit the
[**full pywal spec**](https://github.com/dylanaraps/pywal/wiki/User-Template-Files)

## Variables
- color0
- color1
- color2
- color3
- color4
- color5
- color6
- color7
- color8
- color9
- color10
- color11
- color12
- color13
- color14
- color15
- background
- foreground
- cursor

and it's variants, just append it to the variable name (e.g. `color0.rgb`, `background.blue` ...):
- .rgb
- .rgba
- .xrgba
- .strip
- .red
- .green
- .blue



Miscellaneous variables below are also avaliable, these don't support the variants from above:
- wallpaper
- alpha
- alpha_dec
