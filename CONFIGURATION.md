# CONFIGURATION

todolint loads an optional `todolint.toml` file in the current working directory.

## Example

```toml
# debug = true

# path_exclusion_pattern = "^.*(/|\\\\)?(todolint\\.toml|\\.git|i18n|l10n|node_modules|target|vendor)$"

formal_task_pattern = "(?i)^.*pendiente: [^:]+:.+$"

task_pattern = "(?i)^.*\\b(pte|pend|pendiente)\\b.*$"
```

Above, we see a basic Spanish configuration that overrides the default English `todo` task patterns with more practical ones.
