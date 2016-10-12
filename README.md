# NASH - Nash is Not A Shell

> A tini like process runner pulls environment variables from external services and run your command

## Install

```bash
cargo install nash
```

## Usage

```
Usage:
    nash [options] <source> [<source_args>...] -- <command> [<command_args>...]
Options:
    -h,     --help       Display this message
    -v,     --version    Print version info and exit
    -g,     --group      Forward signals to process group rather than the single process
Sources:
    file                 The local file source
    s3                   File source from s3
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
nash file /path/to/sth.env -- printenv 
```

### From aws S3

When we have a file `s3://path/to/sth.env`, with the content below:

```env
FOO=bar
BAR=baz
```

We can do:

```
nash s3 --bucket path --object /to/sth.env --region ap-southeast-2 -- printenv 
```

## When to use it?

When we start running a service using docker. We always need some kind of keys need to use to connect
to the database or other external services.
