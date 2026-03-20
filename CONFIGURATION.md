# CONFIGURATION

todolint uses [TOML](https://toml.io/en/) syntax.

# todolint.toml

todolint loads an optional `todolint.toml` file in the current working directory.

# debug

Default: `false`

Enables additional logging.

Example:

```toml
debug = true
```

# skip_paths

Default:

```toml
[
    "todolint.toml",
    ".git",
    "i18n",
    "l10n",
    "node_modules",
    "target",
    "vendor",
]
```

Exclude matching file basenames.

Example:

```toml
skip_paths = [
    ".DS_Store",
    "Thumbs.db",
]
```

# formal_task_pattern

Default: `"(?i)^.*pending: [^:]+:.+$"`

Exclude matching lines.

Syntax: [Rust regex](https://crates.io/crates/regex).

Example:

```toml
formal_task_pattern = "(?i)^.*pendiente: [^:]+:.+$"
```

# task_names

Default:

```toml
[
    "band aid",
    "band-aid",
    "bandaid",
    "bodge",
    "cludge",
    "duct tape",
    "duct-tape",
    "ducttape",
    "duck tape",
    "duck-tape",
    "ducktape",
    "hack",
    "kludge",
    "fixme",
    "jury rig",
    "jury-rig",
    "juryrig",
    "macgyver",
    "makeshift",
    "rube goldberg",
    "rube-goldberg",
    "rube goldberg",
    "stop-gap",
    "stop gap",
    "stopgap",
    "temporary solution",
    "to-do",
    "todo",
    "waiting on",
    "workaround",
]
```

Trigger code smell warnings on matching terms.

Case insensitive.

Example:

```toml
task_names = [
    "pte",
    "pend",
    "pendiente",
    "truco",
]
```
