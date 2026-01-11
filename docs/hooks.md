# Hooks
This lets you spawn commands from the config file, reducing fragmentation
and keeping the program more concise, avoiding the creation of multiple scripts.

> [!NOTE]
> Keep in mind that this runs using the default shell in the system located at `/bin/sh`.

## Syntax
Create a TOML table header named `hooks`:
```toml
[hooks]
```

After that, you can add each entry with a unique name.
```toml
[hooks]
notif = 'notify-send "New Scheme Generated!"'
```

You can use:
- `' '` for simple scripts
- `" "` when you require to call another program with `$()` and for escaping `\` with `\\`
- `""" """` Triple double quotes for raw strings, when you need to include double quotes inside command.

## Example: Adding a wallpaper
The simplest yet useful example is to change the wallpaper to a random one
whenever you run wallust.
```toml
wallpaper = """swww img "$(find ~/Media/Pictures/Wallpapers -iregex '.*.\\(jpg\\|jpeg\\|png\\|gif\\)' -type f | shuf -n 1)" """
```
You can replace `swww img` which whatever wallpaper program you are using.


## Disabling
Use the cli flag `--no-hooks` to ignore all of the above, avoids calling `/bin/sh`.
Alternatively, add `no_hooks` to the config file as follows:
```toml
no_hooks = true
```
And, no matter how many hooks are there, they won't run.
