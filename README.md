# todolint: a static analyzer for incomplete code smells

![logo](examples/pencils.png)

# SUMMARY

todolint scans projects for comments about unfinished tasks.

* `TODO:`...
* `FIXME:`...
* Etc.

# EXAMPLES

```console
$ cd examples

$ todolint .
docs/backlog.txt:1:TODO: Internationalize console messages.
greet.c:4:// FixMe: Validate argc
```

See `todolint -h` for more options.

# ABOUT

todolint encourages high quality software projects.

Remark that comments like `TODO`... belong outside of version control, in a ticketing system. This keeps your codebase smaller, more focused, easier to manage.

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
