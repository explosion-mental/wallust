# Templates

A **template** is simply a file that has _placeholders_ in order to replace
them with values. In wallust case, these values can range from either the
colors generated, the image/theme served or the backend used. These values are
represented by variables, which you can [look up](./variables.md) inside
placeholders.

By using templates you can apply the colors to a program that uses a config file.


I've made some templates for some known programs at
<https://codeberg.org/explosion-mental/wallust-templates>. If you have a
template you want to share, that is the place.

## Where?

The default templates directory is at `CONFIG/wallust/templates`, where `CONFIG` changes depending the platform wallust is ran at (e.g. `$XDG_CONFIG_HOME` in linux). Check out [`features`](../intro.md) section.


You can also change this in cli with `--templates-dir`.
