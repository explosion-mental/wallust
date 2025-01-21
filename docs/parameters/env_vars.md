# Enviromental Variables in templates/targets paths

Allows you to have shell variables in your paths. For example:

```toml
# using XDG var
pywal = { src = "pywal", dst = "${XDG_HOME_CONFIG}/templates/pywal-result/", pywal = true }
```


To avoid possible security issues, this flag is **disabled** by default.

<hr>

To edit this value:
- **Config file**: `env_vars = true`
