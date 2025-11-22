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

$ ls -l
total 8
drwxr-xr-x  5 andrew  staff  160 Nov 22 11:16 en-us
drwxr-xr-x  6 andrew  staff  192 Nov 22 12:54 es-mx
drwxr-xr-x  6 andrew  staff  192 Nov 22 13:07 zh-cn
drwxr-xr-x  6 andrew  staff  192 Nov 22 13:09 zh-hk

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

$ cd ../zh-cn
$ todolint .
docs/backlog.txt:1:待办: 将控制台消息国际化为普通话。
greet.c:4:// 待办: 验证 1 < argc < 3
greet.c:8:// 待办
metrics.js:10:// 妙招: 零

$ todolint .
docs/backlog.txt:1:待辦: 國際化控制台訊息。
greet.c:4:// 待辦: 驗證 1 < argc < 3
greet.c:8:// 待辦
metrics.js:10:// 妙招: 零
```

See `todolint -h` for more options.

# CONFIGURATION

For more details on configuring todolint, see [CONFIGURATION.md](CONFIGURATION.md).

# ABOUT

todolint identifies software bugs, by searching codebases for mentions of unresolved `TODO` tasks.

Examples:

* `pend: pasear al perro`
* `pend.: pasear al perro`
* `pte: pasear al perro`
* `todo: walk the dog`
* `裏技: 犬の散歩`
* `粗笨: 遛狗`

However, todolint allows comments that cite a URI-like resource, using the notation `<status>: <uri>`. The resource may provide a FAQ, ticketing system, or other documentation.

Exception:

* `pending: https://pubs.opengroup.org/onlinepubs/9799919799/`

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
