# DEVELOPMENT GUIDE

todolint follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting, managing multiplatform Docker images, and so on, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [awscli](https://aws.amazon.com/cli/)
* [bash](https://www.gnu.org/software/bash/) 4+
* [Docker](https://www.docker.com/)
* POSIX compliant [findutils](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/find.html)
* [jq](https://jqlang.org/)
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [rustup](https://rustup.rs/) 1.28.1+
* [Rust](https://www.rust-lang.org/en-US/)
* [cross](https://crates.io/crates/cross) 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
* GNU [tar](https://www.gnu.org/software/tar/) as `gtar`
* Provision additional dev tools with `make -f install.mk`

## Recommended

* Apple Silicon macOS users may want to apply `DOCKER_DEFAULT_PLATFORM=linux/amd64`, in order to account for images commonly lacking `linux/arm64` buildx platforms
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)

# INSTALL BINARIES FROM SOURCE

```sh
make install
```

# UNINSTALL BINARIES

```sh
make uninstall
```

# SECURITY AUDIT

```sh
make audit
```

# LINT

```sh
make lint
```

# TEST

```sh
make test
```

# CROSSCOMPILE BINARIES

```sh
make crit
```

# ARCHIVE BINARIES

```sh
make port
```

# PACKAGE BIARIES

```sh
make package
```

# PUBLISH CRATE

```sh
make publish
```

# BUILD DOCKER IMAGES

```sh
make docker-build
```

# TEST PUSH DOCKER IMAGES

```sh
make docker-test
```

# PUSH DOCKER IMAGES

```sh
make docker-push
```

# CLEAN

```sh
make clean
```
