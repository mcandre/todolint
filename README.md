# todolint: code comment SAST security scanner

[![CloudFlare R2 install media downloads](https://img.shields.io/badge/Cloudflare-F28220?style=for-the-badge&logo=Cloudflare&logoColor=white&style=flat)](#download) [![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/todolint)](https://hub.docker.com/r/n4jm4/todolint) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/todolint?label=crate%20downloads)](https://crates.io/crates/todolint) [![GitHub Downloads](https://img.shields.io/github/downloads/mcandre/todolint/total?logo=github)](https://github.com/mcandre/todolint/releases) [![docs.rs](https://img.shields.io/docsrs/todolint)](https://docs.rs/todolint/latest/todolint/) [![Test](https://github.com/mcandre/todolint/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/todolint/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-0)](LICENSE.md)

![pencil case logo](todolint.png)

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

# DOWNLOAD

<table>
  <thead>
    <tr>
      <th>OS</th>
      <th colspan=2>Package</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Alpine Linux 3.23+</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/alpine-linux/todolint-0.0.9-r1.x86_64.apk">Intel</a></td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/alpine-linux/todolint-0.0.9-r1.aarch64.apk">ARM</a></td>
    </tr>
    <tr>
      <td>Fedora 43+</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/fedora/todolint-0.0.9-1.x86_64.rpm">Intel</a></td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/fedora/todolint-0.0.9-1.aarch64.rpm">ARM</a></td>
    </tr>
    <tr>
      <td>FreeBSD 13</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/freebsd-amd64/todolint-0.0.9_1.pkg">Intel</a></td>
      <td></td>
    </tr>
    <tr>
      <td>macOS 26 Tahoe+</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/macos/todolint-x86_64-0.0.9-1.pkg">Intel</a></td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/macos/todolint-arm64-0.0.9-1.pkg">ARM</a></td>
    </tr>
    <tr>
      <td>NetBSD 10.1</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/netbsd-x86_64/todolint-0.0.9nb1.tgz">Intel</a></td>
      <td></td>
    </tr>
    <tr>
      <td>Ubuntu 24.04 Noble+</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/ubuntu/todolint_0.0.9-1_amd64.deb">Intel</a></td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/ubuntu/todolint_0.0.9-1_arm64.deb">ARM</a></td>
    </tr>
    <tr>
      <td>Windows 11+</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/windows/todolint-0.0.9.1-x64.msi">Intel</a></td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.9/windows/todolint-0.0.9.1-arm64.msi">ARM</a></td>
    </tr>
  </tbody>
</table>

# SYSTEM REQUIREMENTS

## Bitness

64

For more platforms and installation methods, see our [install guide](INSTALL.md).

# RESOURCES

* [mcandre/linters](https://github.com/mcandre/linters) curates many linters, SAST tools, and style guides

✏️
