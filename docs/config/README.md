# Configuration File

While `wallust` can work out **without a config file**, it results useful to
define constant options in a file than giving them each time as a cli flag.
This is why **all parameters are optional**.

Without a config file, `wallust` will choose to default implementations, which
are explained in detail in the [parameters](../parameters/README.md) section.

That being said, you can start editing your config file.

## Format
The chosen format for the config file is _Tom's Obvious Minimal Language_ (TOML).

You can check the full specification [here](https://toml.io/en/v1.0.0).

## Structure
The config file is divided into two parts:
- `global` space
- `templates` space

Inside the **global** space you can define any
[parameter](../parameters/README.md) that you want.

To enter the **templates** space, however, requires a `[templates]` header.
Below this, you can only define templates, which is explained in the next page.
