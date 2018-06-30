# Scroller

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
