# ce-wasi-example

This example shows how to use [CloudEvents](https://cloudevents.io/) with [WebAssembly WASI](https://wasi.dev/) and [Krustlet](https://krustlet.dev/)

### Add Krustlet node

Add a Krustlet node to your local Kubernetes cluster

```sh
$ git clone https://github.com/krustlet/krustlet.git
$ cd krustlet
$ scripts/bootstrap.sh
$ KUBECONFIG=~/.krustlet/config/kubeconfig target/debug/krustlet-wasi --port 3000 --bootstrap-file ~/.krustlet/config/bootstrap.conf
```

### Build example

Build this example and make it target `wasm32-wasi` target.

```sh
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
```

### Push OCI image

Now we need to create an OCI image from our binary

```sh
wasm-to-oci push target/wasm32-wasi/release/ce-wasi-example.wasm ghcr.io/dejanb/ce-wasi-example:latest
```

### Run example

You can deploy this image to the K8s cluster with an example shown in [k8s.yaml]

```sh
kubectl apply -f k8s.yaml
```

### Monitor results

Check if the workload is running correctly

```sh
kubectl logs -f -p ce-wasi-example
```

or

```sh
stern ce-wasi-example
```

### Demo

Recording of a demo is available at https://www.youtube.com/watch?v=KBFh_X2N2vA