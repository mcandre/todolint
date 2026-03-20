# DEVELOPMENT

We follow standard, `cargo` based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, we further supplement with some software industry tools.

# DEV ENVIRONMENT

## Prerequisites

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [awscli](https://aws.amazon.com/cli/)
* [bash](https://www.gnu.org/software/bash/) 4+
* [Docker](https://www.docker.com/)
* [jq](https://jqlang.org/)
* [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [Rust](https://www.rust-lang.org/en-US/)
* [cross](https://crates.io/crates/cross) 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
* Provision additional dev tools with `make -f install.mk`

## Recommended

* [asdf](https://asdf-vm.com/)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

# TASKS

We automate engineering tasks.

## Build

```sh
make
```

## Install

```sh
make install
```

## Uninstall

```sh
make uninstall
```

## Security Audit

```sh
make audit
```

## Lint

```sh
make lint
```

## Test

```sh
make test
```

## Crosscompile Binaries

```sh
make crit
```

## Package Binaries

```sh
make package
```

## Upload Packages

```sh
make upload
```

## Publish Crate

```sh
make publish
```

## Clean Workspace

```sh
make clean
```
