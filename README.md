# Scroller
```text
scroller 0.1.3
Davide Giovannini <giovannini.davide90@gmail.com>
A simple rust client to perform scroll search requests to an ElasticSearch cluster.

USAGE:
    scroller [FLAGS] [OPTIONS] <host> <index> <output> [source]...

FLAGS:
    -h, --help       Prints help information
    -p, --pretty     pretty print output
    -s, --silent     hide the progressbar
    -V, --version    Prints version information

OPTIONS:
    -l, --limit <limit>    get at most <limit> results
    -q, --query <query>    path to a json file containing the query to use (defaults to match_all)

ARGS:
    <host>         ElasticSearch host, protocol and/or port can be omitted if they are respectively "http" or
                   ":9200"
    <index>        Index to scroll
    <output>       path of the output jsonl file (use - to output to stdout instead)
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

To create a static linux binary

```bash
docker container run --rm -v $PWD:/volume --name musl -it clux/muslrust cargo install --force --root . --path .
```
