# Layer limit calculator

This program performs a binary search to find the point at which we reach a layer limit when running
container images. It assumes you are able to run docker without root access (alternatively, you can
run the compiled binary using `sudo ./target/release/layer-limit-calculator`).

### Usage

```
cargo run --release
```

### Tips

The program crashes when using stock Docker, because the image builder fails due to the high number
of layers. Consider [configuring Docker to use containerd](https://docs.docker.com/storage/containerd/#enable-containerd-image-store-on-docker-engine),
which is also used by stock Kubernetes.

You could also write a program that generates images directly, without going through Docker, but
we chose to use Dockerfiles here to keep things simple (even though it takes longer to run).

## Results

We were able to run a successful test with __260 layers__, and suspect that the limit is even higher.
However, at higher layer counts the Docker layer builder hits an internal error, so we couldn't test
them. A possible workaround for this could be to build the images programmatically, in Rust code,
instead of going through Docker.

Software versions:

```
$ docker --version
Docker version 26.1.2, build 211e74b

$ containerd --version
containerd containerd.io 1.6.31 e377cd56a71523140ca6ae87e30244719194a521
```
