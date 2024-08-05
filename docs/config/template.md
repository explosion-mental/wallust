# Defining a template in the config file

Templates are optional and defined inside the `[templates]` header. Here it's
recommended to use single quotes (`'`) instead of double quotes (`"`) since the
first one, by the toml format, ignores backslashes (`\`) as escape codes,
allowing you to define Widows like paths, e.g. `'C:\Users\Desktop\'`.

## template
A relative path that points to a file where wallust.toml is located, usually at
`~/.config/wallust/templates`. This file can also be a directory, which will be
templated **non-recursively** (only the first recursion, like `du ... --max-depth 1`)

Check out [`templates`](../templates/README.md) section for more.

## target
Absolute path in which to place a file with generated templated values. This
field CAN expand the `~` as the `$HOME` enviromental variable. If
**template** is a directory, this **must** correspond and be one.

## pywal _(optional)_
Indicates to treat **template** as a pywal template, using `{variable}` syntax. (default: **false**)

<div class="warning">
This is mostly unstable.

While the implementation mostly works out, it isn't a garantee that it will
work all the time. If you are making a new template, make sure you do it with
the [new syntax](../template/README.md)
</div>
