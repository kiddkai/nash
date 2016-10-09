# NASH - Nash is Not A Shell

> A tini like process runner pulls environment variables from external services and run your command

## Install

```bash
cargo install nash
```

## Usage

```
Usage:
    nash [--from=<URL>] <cmd> [<args>...]
Options:
    -h,     --help       Display this message
    -V,     --version    Print version info and exit
    -v,     --verbose    Use verbose output
    -g,     --group      Forward signals to process group rather than the single process
    -f URL, --from=URL   URI to retrive the environments from
```

## Sources

### Local file system

When we have a file `/path/to/sth.env`, with the content below:

```env
FOO=bar
BAR=baz
```

We can do:

```
nash --from=file:///path/to/sth.env sh -c 'echo "$FOO|$BAR"'
```

Which prints:

```
bar|baz
```

## When to use it?

When we start running a service using docker. We always need some kind of keys need to use to connect
to the database or other external services.
