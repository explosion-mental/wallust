# Defining a template in the config file

Templates are optional and defined inside the `[templates]` header. Here it's
recommended to use single quotes (`'`) instead of double quotes (`"`) since the
first one, by the toml format, ignores backslashes (`\`) as escape codes,
allowing you to define Widows like paths, e.g. `'C:\Users\Desktop\'`.

## template
A relative path that points to a file where wallust.toml is located, usually at
`~/.config/wallust/templates`. This file can also be a directory, which will be
templated **recursively**. If you want to avoid recursion, see [max_depth](#max_depth) below.

Check out [`templates`](../templates/README.md) section for more.

## target
Absolute path in which to place a file with generated templated values. This
field CAN expand the `~` as the `$HOME` enviromental variable. If
**template** is a directory, this **must** correspond and be one.

## pywal _(optional)_
Indicates to treat **template** as a [pywal template](../templates/pywal.md), using `{variable}` syntax. (default: **false**)

## max_depth
This is variable is optional, by default disabled and thus, doesn't limit recursion.
When enabled, it accepts a number that indicates the quantity of recursion
level to accept, similar to `du ... --max-depth 1` (and probably other utils).

This variable only has effect when the **template and target are directories**.
_Remember_ anyway that **if `template` is a directory, `target` SHOULD also be one.**
