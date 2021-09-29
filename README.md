# ce-wasi-example

This example shows how to use CloudEvents with Wasm WASI and Krustlet

### Add Krustlet node

```sh
scripts/bootstrap.sh
KUBECONFIG=~/.krustlet/config/kubeconfig target/debug/krustlet-wasi --port 3000 --bootstrap-file ~/.krustlet/config/bootstrap.conf
```

### Build example

```sh
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
```

### Push OCI image

```sh
wasm-to-oci push target/wasm32-wasi/release/ce-wasi-example.wasm ghcr.io/dejanb/ce-wasi-example:latest
```

### Run example

```sh
kubectl apply -f k8s.yaml
```

### Monitor results

```sh
kubectl logs -f -p ce-wasi-example
```

or

```sh
stern ce-wasi-example
```
