# Pywal Template Engine
You can enable pywal like syntax in the config file with `pywal = true`, check out [defining a template in the config file](https://explosion-mental.codeberg.page/wallust/config/template.html#pywal-optional).

The syntax is simple, but more variables are added given that it's engine and
spec doesn't support runtime evaluation functions.

Don't forget to visit the
[**full pywal spec**](https://github.com/dylanaraps/pywal/wiki/User-Template-Files) for more
details, as this engine will try to keep it without changes, but here is a quick tutorial.

## Syntax
The syntax logic is simply "Find and Replace" like:

```
somevariable = {color2}
anothervariable = {color8.rgb}
```
and to escape braquest simply add one more than desired:
```
// Example snippet.
* {{
    active-background: {color2};
    active-foreground: {foreground};
    normal-background: {background};
    normal-foreground: {foreground};
    urgent-background: {color1};
    urgent-foreground: {foreground};
    // ...
}}
```

## Variables
Below is a simple list with possible variables:
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
- .alpha



Miscellaneous variables below are also avaliable, these don't support the variants from above:
- wallpaper
- alpha
- alpha_dec

