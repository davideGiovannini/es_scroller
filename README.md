# Scroller

```text
scroller 0.1.0
Davide Giovannini <giovannini.davide90@gmail.com>
An example of StructOpt usage.

USAGE:
    scroller [FLAGS] [OPTIONS] <host> <index> <output> [source]...

FLAGS:
    -h, --help       Prints help information
    -p, --pretty     pretty print output
    -s, --silent     hide the progressbar
        --stream     Stream output to stdout
    -V, --version    Prints version information

OPTIONS:
    -l, --limit <limit>    get at most <limit> results
    -q, --query <query>    path to a json file containing the query to use (defaults to match_all)

ARGS:
    <host>         Url and port of the elastic search host
    <index>        Index to scroll
    <output>       path of the output jsonl file (only if not using --stream)
    <source>...    _source  fields
```

## Testing

To spin up the testing environment
```bash
docker-compose -f tests/docker-compose.yml up -d 
```

To run the tests
```bash
cargo test
```

To tear it down 
```bash
docker-compose -f tests/docker-compose.yml down -v --rmi local 
```

## Building a static binary

Copy the project to the docker daemon host
```bash
vagrant scp <path-to-project> /tmp/scroller
```

Make sure to have the docker image
```bash
docker pull clux/muslrust
```

Build the project
```bash
docker container run --rm -v /tmp/scroller:/volume --name musl -it clux/muslrust cargo install --root .
```

Copy back the binary
```bash
vagrant scp :/tmp/scroller/bin/scroller .
```
