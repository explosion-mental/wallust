# Template Syntax

You can use **wallust** generated colors in a program by _templating_ the
colors in it's config file. Here is an overview of the general syntax of a
template.

You reference variables in the following syntax:

```
{{color0}}
```

For applying a **filter** you use the _pipe_ character (`|`) like this:

```
{{background | strip}}
```

And if the filter requires an argument:

```
{{background | lighten(0.3)}}
```

Remember that filters require a valid type to **apply to** in these examples we
are using colors, which can even be defined literally:

```
{{ "#4ff4ff" | lighten(0.3)}}
```

For **both**, being applied to or as an argument of a filter:

```
{{ color2 | blend("4ff4ff")}}
```

If you need to write a literal `{{`, that doesn't references any variable, you
can write literals inside the delimiters:

```
{{ "{{" }} {{ "}}" }}
```

You can also use control flow expressions with `{% %}` delimiters:

```
{% if backend == "wal" %}
I am using the '{{backend}}' backend, getting a pywal like scheme.
{% elif backend == "fastresize" %}
This backend is called "{{palette}}" and, uses SIMD optimizations and is so fast!
{% else %}
I don't care about any other backends. Be happy!
{% endif %}
```

Or inline them:

```
{{ "I'm using the kmeans algo!" if backend == "kmeans" else "Some backend is in use" }}
```

Since mostly everything can be represented as a string (we've seen how colors are represented),
indexing results very useful! The syntax for indexing is basically the Python one.

```
{# I'll hardcode a color based on the palette being used. #}
{% if palette[:4] == "dark" %}
somevariable = "#eeffbb"
{% else %}
somevariable = "#aabbee"
{% endif %}
```

And yes, you can comment inside your template, the comments won't be rendered in the final target file:

```
{# This won't be visible! #}
```

There are more control flow instructions, like the for loop:

```
{# This will generate color0 = .. to color18,
since `colors` contains background, foreground and cursor variables #}
{% for c in colors %}
color{{- loop.index }} = {{c-}}
{% endfor %}
```

You can add a minus sign (-) at the start or the end of the delimiters to supress [**_vertical spacing_**](http://jinja.pocoo.org/docs/templates/#whitespace-control) (White space control with the minus sign `-`)

The syntax comes from the library being used, which is _minijinja_, a subset of the template engine 'Jinja2'.

You can read more at:
[**Jinja2 official syntax**](https://jinja.palletsprojects.com/en/2.10.x)
and contrast features with the supported syntax at
[**Compatibility of minijinja**](https://github.com/mitsuhiko/minijinja/blob/main/COMPATIBILITY.md)

# Example

```
# zathurarc config sample
...

# colors
set default-bg     "{{background}}"
set default-fg     "{{foreground}}"

# make it a bit lighter than background
set statusbar-bg   "{{background | lighten(0.3)}}"

# make it darken by blending to a darken color
set statusbar-fg   "{{foreground | blend("#eeeeee")}}"

# use it's complementary
set inputbar-bg    "{{background | complementary}}"
```

Then you can add this file to `~/.config/wallust`
and use the config file to **template** it. For example,
`zathura.template = ~/.config/wallust/zathurarc`, and then define a
**target** field, see wallust (5).
