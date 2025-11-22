# todolint: a static analyzer for incomplete code snippets

![logo](todolint.png)

# SUMMARY

todolint scans projects for comments about incomplete source code snippets.

* `hack`
* `fixme`
* `todo`
* etc.

# EXAMPLES

```console
$ cd examples

$ ls
en-us	es-mx	zh-cn	zh-hk

$ cd en-us
$ todolint .
docs/backlog.txt:1:FIXME: Internationalize console messages.
greet.c:4:// TODO: Validate 1 < argc < 3
greet.c:8:// TODO
metrics.js:10:// hack: divide by zero

$ cd ../es-mx
$ todolint .
docs/backlog.txt:1:PTE: Internacionalizar los mensajes de la consola.
greet.c:8:// PTE: Validar 1 < argc < 3
greet.c:12:// PTE
metrics.js:10:// truco: dividir por cero"

$ cd ../zh-cn
$ todolint .
docs/backlog.txt:1:待办: 将控制台消息国际化为普通话。
greet.c:4:// 待办: 验证 1 < argc < 3
greet.c:8:// 待办
metrics.js:10:// 妙招: 零除

$ todolint .
docs/backlog.txt:1:待辦: 國際化控制台訊息。
greet.c:4:// 待辦: 驗證 1 < argc < 3
greet.c:8:// 待辦
metrics.js:10:// 妙招: 零除
```

See [CONFIGURATION.md](CONFIGURATION.md) for configuration file options.

Run `todolint -h` for CLI options.

# ABOUT

todolint identifies software bugs, by searching codebases for mentions of unresolved `TODO` tasks.

Examples:

```rust
// todo: walk the dog

// todo
// walk the dog
```

However, todolint allows comments that cite a URI-like resource, using the notation `<status>: <uri>`. The resource may provide a FAQ, ticketing system, or other documentation.

Exception:

```rust
// pending: https://doc.rust-lang.org/beta/rustc/platform-support.html
```

Thus, we now have a formal notation to track coding imperfections, including metadata about _why_ the code may be stuck indefinitely in its current written state.

# CRATE

https://crates.io/crates/todolint

# API DOCUMENTATION

https://docs.rs/todolint/latest/todolint/

# DOWNLOAD

https://github.com/mcandre/todolint/releases

# INSTALL FROM SOURCE

```console
$ cargo install --force --path .
```

# RUNTIME REQUIREMENTS

(None)

# CONTRIBUTING

For more details on developing todolint itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# LICENSE

BSD-2-Clause

✏️
