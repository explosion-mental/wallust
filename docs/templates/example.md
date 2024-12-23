# Example

```
# zathurarc config sample
...

# colors
set default-bg     "{{background}}"

# complementary foreground, but keep it light
set default-fg     "{{foreground | complementary | lighten(0.5)}}"

# make it a bit lighter than background
set statusbar-bg   "{{background | lighten(0.3)}}"

# make it darken by blending to a darken color
set statusbar-fg   "{{foreground | blend("#eeeeee")}}"

# use it's complementary
set inputbar-bg    "{{background | complementary}}"
```

Then you can add this file to `~/.config/wallust/templates`
and use the config file to **template** it. For example,
`zathura.template = 'zathurarc'`, and then define a
**target** field, see [config](../config/README.md).
