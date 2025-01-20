# Replacement for `pywal`

With the subcommand `wallust pywal`, wallust accepts command line arguments the same as the `wal` tool.

This desicion was made given that there are [many other proyects](https://github.com/dylanaraps/pywal/wiki/Plugins)
that aim to use `pywal`. To avoid burdening with multiple templates engines and
syntax from
[many other proyects that create colorschemes](https://codeberg.org/explosion-mental/wallust#related),
a simple "standard"ish way is to keep the most popular template syntax, because
it's tool is the most popular (even tho it's archived): **pywal**.

So, if you are moving **from** `wal` to `wallust`, this page might help you
avoid extra work and continue your workflow.

## Alias to a script
First, you can change `pywal` with `wallust pywal` by making a script with the `wal` name.

```sh
#!/bin/sh
exec wallust pywal "$@"
```

Now, all programs that require `wal`, will use `wallust pywal` in the background.

To modify wallust behaviour in `pywal` mode, simply use the config file, since, currently, all `pywal` flags are ignored.
