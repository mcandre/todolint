# INSTALL GUIDE

In addition to OS packages, todolint also supports alternative installation methods.

# INSTALL (CARGO)

todolint is packaged as a Rust crate.

```sh
cargo install todolint
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)

# INSTALL (CURL)

curl based installs automatically download and extract precompiled binaries.

```sh
curl -L https://raw.githubusercontent.com/mcandre/todolint/refs/heads/main/install-todolint | sh
```

## Postinstall

Ensure `$HOME/.local/bin` is registered with your shell's `PATH` environment variable.

## Uninstall

```sh
curl -L https://raw.githubusercontent.com/mcandre/todolint/refs/heads/main/uninstall-todolint | sh
```

## System Requirements

### Bitness

64

### Operating Systems

* FreeBSD 13 (Intel)
* Illumos (Intel)
* Linux (ARM, Intel)
* macOS 26 Tahoe+ (ARM, Intel)
* NetBSD 10.1 (Intel)
* WSL 2 (ARM, Intel)

### Prerequisites

* [bash](https://www.gnu.org/software/bash/) 4+
* [curl](https://curl.se/)

# INSTALL (PRECOMPILED BINARIES)

Precompiled binaries may be installed manually.

## Install

1. Download a [tarball](https://github.com/mcandre/todolint/releases) corresponding to your environment's architecture and OS.
2. Extract executables into a selected directory.

   Examples:

   * `~/.local/bin` (XDG compliant per-user)
   * `/usr/local/bin` (XDG compliant global)
   * `~/bin` (BSD)
   * `~\AppData\Local` (native Windows)

## Postinstall

Ensure the selected directory is registered with your shell's `PATH` environment variable.

## Uninstall

Remove the application executables from the selected directory.

## System Requirements

### Bitness

64

### Operating Systems

* FreeBSD 13 (Intel)
* Illumos (Intel)
* Linux (ARM, Intel)
* macOS 26 Tahoe+ (ARM, Intel)
* NetBSD 10.1 (Intel)
* Windows 11+ (ARM, Intel)

# INSTALL (COMPILE FROM SOURCE)

todolint may be compiled from source.

```sh
git clone https://github.com/mcandre/todolint.git
cd todolint
cargo install --force --path .
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)
* [git](https://git-scm.com/)

For more details on developing todolint, see our [development guide](DEVELOPMENT.md).
