# Layer limit calculator

This program performs a binary search to find the point at which we reach a layer limit when running
container images. It assumes you are able to run docker without root access.

### Usage

```
cargo run --release
```

### Tips

If you are using Docker with the default backend, you should find the limit around 127 layers. If
you are using it with the containerd backend, the limit seems to be much higher (I wasn't able to
test it because the docker build started failing).

You could also write a program that generates images directly, without going through Docker, but
we chose to use a Dockerfile template here to keep things simple (even though it takes longer to
run).
