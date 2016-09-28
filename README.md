# NASH - Nash is Not A Shell

> A tini like process runner pulls environment variables from external services and run your command

## Install

```bash
cargo install nash
```

In `OSX`, when you get a error message like this:

```
cargo:warning=src/openssl_shim.c:1:10: fatal error: 'openssl/hmac.h' file not found
cargo:warning=#include <openssl/hmac.h>
cargo:warning=         ^
cargo:warning=1 error generated.
ExitStatus(ExitStatus(256))
```

A easy way to fix this is using `homebrew` is run a simple command:

```
brew link --force openssl
```

If you are not using `homebrew`, please checkout this [issue](https://github.com/sfackler/rust-openssl/issues/255).

## Usage

```
nash

Command runner

Usage:
    nash <command> [<args>...]
    nash [options]

Options:
    -h, --help          display this message
    -V, --version       display current version
    --list              list commands
    -v, --verbose       Use verbose output

Commands:
    run                 Run the command
    init                Create a nash config
    help                Show helps for different commands
```