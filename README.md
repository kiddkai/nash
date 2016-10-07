# NASH - Nash is Not A Shell

> A tini like process runner pulls environment variables from external services and run your command

## Install

```bash
cargo install nash
```

## Usage

```
Usage:
    nash [options] <cmd> [<args>...]
Options:
    -h, --help       Display this message
    -V, --version    Print version info and exit
    -v, --verbose    Use verbose output
    -g, --group      Forward signals to groups
    --from           URL to retrive the environments from
    --from-env       Envrionment variable contains the URL to retrive the environments from
Examples:
    nash ls -al
    nash --from s3://bucket/secrets/foo.json ls -al
    nash --from-env NASH_FROM ls -al
```

## When to use it?

When we start running a service using docker. We always need some kind of keys need to use to connect
to the database or other external services.
