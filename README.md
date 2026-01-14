# todolint: code comment SAST security scanner

![pencil case logo](todolint.png)

[![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/todolint)](https://hub.docker.com/r/n4jm4/todolint) [![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/todolint?label=crate%20downloads)](https://crates.io/crates/todolint) [![docs.rs](https://img.shields.io/docsrs/todolint)](https://docs.rs/todolint/latest/todolint/) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/GUMROAD-36a9ae?style=flat&logo=gumroad&logoColor=white)](https://mcandre.gumroad.com/)

# SUMMARY

todolint identifies bugs based on code comments.

* `hack`
* `fixme`
* `todo`
* etc.

# LOCALIZATIONS WELCOME

Speakers are invited to submit pull requests to improve the writing in our examples.

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

$ cd zh-hk
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

# INSTALLATION

See [INSTALL.md](INSTALL.md).

# SEE ALSO

* [chandler](https://github.com/mcandre/chandler) normalizes tarballs
* [kirill](https://github.com/mcandre/kirill) scans JSON documents
* [linters](https://github.com/mcandre/linters) curates many linters, SAST tools, and style guides
* [nile](https://github.com/mcandre/nile) normalizes ebooks
* [slick](https://github.com/mcandre/slick) scans POSIX shell scripts
* [stank](https://github.com/mcandre/stank) scans shell scripts
* [unmake](https://github.com/mcandre/unmake) scans makefiles

✏️
