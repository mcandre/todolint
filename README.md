# todolint: a static analyzer for incomplete code snippets

![logo](examples/pencils.png)

# SUMMARY

todolint scans projects for comments about incomplete source code snippets.

* `HACK:`
* `FIXME:`
* `TODO:`
* Etc.

# EXAMPLES

```console
$ cd examples

$ todolint .
docs/backlog.txt:1:FIXME: Internationalize console messages.
greet.c:4:// TODO: Validate argc >= 1
greet.c:7:// TODO: Validate argc <= 2
greet.c:10:// PTE: falta codigo de salida
metrics.js:11:// hack: divide by zero
```

See `todolint -h` for more options.

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
