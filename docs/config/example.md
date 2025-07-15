# Configuration Sample

Below is a simple example exahusting all possible cases (syntax wise) in the
`[templates]` header. All the format is correct:

```toml
# Let's keep good old pywal look and feel
backend = "wal"
#color_space = "lch" # idc about this one..
#threshold = "20" # neither about this, since I read wallust does it automagically..
# classic look
palette = "dark16"
# let's keep the contrast very very very clear.
check_contrast = true

[templates]
# dunst templates
dunst.template = "dunstrc.monitor"
dunst.target = "~/.config/dunst/dunstrc"

# one liner for zathura
zathura = { template = 'zath', target = '~/.config/zathura/zathurarc' }

# even a shorter way, using directories, but only one level recursion
glava = { src = 'glava', dst = '~/.config/glava/', max_depth = 1 }

# or splited in the dotted syntax
res.src = "xres"
res.dst = "~/.config/Xresources"

# old times, good times. Here I put old pywal templates.
# NOTE THAT BOTH scr AND dst ARE DIRECTORIES!
pywal = { src = "templates/", dst = '~/.cache/wal/', pywal = true }
```
