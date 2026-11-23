# todolint: code comment SAST security scanner

[![CloudFlare R2 Downloads](https://badgen.net/static/download/Packages/f38020?icon=cloudflare)](#download) [![Crate Downloads](https://badgen.net/crates/d/todolint?label=crate+downloads)](https://crates.io/crates/todolint) [![API Docs](https://badgen.net/static/docs/API/3eca2a?icon=docs.rs)](https://docs.rs/todolint/latest/todolint/) [![CI/CD](https://badgen.net/github/checks/mcandre/todolint?icon=github)](https://github.com/mcandre/todolint/actions) [![License](https://badgen.net/static/license/BSD/ab2b29?icon=freebsd)](LICENSE.md) [![AI](https://badgen.net/static/ai/Gemini|hint?color=886fbf&icon=google-gemini)](AI-DECLARATION.md)

![pencil case logo](todolint.png)

# SUMMARY

todolint identifies bugs based on code comments.

* `hack`
* `fixme`
* `todo`
* etc.

# EXAMPLES

```console
% cd examples/en-us

% todolint .
docs/backlog.txt:1:FIXME: Internationalize console messages.
greet.c:4:// TODO: Validate 1 < argc < 3
greet.c:8:// TODO
metrics.js:10:// hack: divide by zero
```

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
      <td>macOS 26 Tahoe+</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.10/macos/todolint-x86_64-0.0.10-1.pkg">Intel</a></td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.10/macos/todolint-arm64-0.0.10-1.pkg">ARM</a></td>
    </tr>
    <tr>
      <td>Ubuntu 24.04 Noble+ / WSL 2+</td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.10/ubuntu/todolint_0.0.10-1_amd64.deb">Intel</a></td>
      <td><a href="https://pub-b11eab734c5a41ddb16f0a2e0e012d1d.r2.dev/todolint-0.0.10/ubuntu/todolint_0.0.10-1_arm64.deb">ARM</a></td>
    </tr>
  </tbody>
</table>

For more platforms and installation methods, see [INSTALL](INSTALL.md).

For details on tuning todolint, see [CONFIGURATION](CONFIGURATION.md).

For details on building from source, see [DEVELOPMENT](DEVELOPMENT.md).

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

# LOCALIZATIONS WELCOME

Speakers are invited to submit pull requests to improve the writing in our examples.

# RESOURCES

* [mcandre/linters](https://github.com/mcandre/linters) curates many linters, SAST tools, and style guides

✏️
